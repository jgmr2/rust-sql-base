use axum::{extract::{Query, State}, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{collections::HashMap, env, sync::Arc, time::{Duration, Instant}};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    data: Vec<User>,
    timestamp: Instant,
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

async fn get_users(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Value> {
    // Obtener limit desde query params o usar default 100
    let limit = params
        .get("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(100)
        .min(1000); // Máximo 1000 por seguridad

    // Crear clave de cache única para estos parámetros
    let cache_key = format!("users_limit_{}", limit);
    let cache_duration = Duration::from_secs(300); // 5 minutos

    // Intentar obtener desde cache local primero
    {
        let cache = state.cache.read().await;
        if let Some(entry) = cache.get(&cache_key) {
            if entry.timestamp.elapsed() < cache_duration {
                return Json(json!({
                    "status": "success",
                    "data": entry.data,
                    "count": entry.data.len(),
                    "limit": limit,
                    "cached": true
                }));
            }
        }
    }

    // Si no está en cache o expiró, consultar la base de datos
    match sqlx::query_as::<_, User>(
        "SELECT id, name, email, created_at FROM users ORDER BY created_at DESC LIMIT $1"
    )
    .bind(limit)
    .fetch_all(&state.db)
    .await
    {
        Ok(users) => {
            // Guardar en cache local
            {
                let mut cache = state.cache.write().await;
                cache.insert(cache_key, CacheEntry {
                    data: users.clone(),
                    timestamp: Instant::now(),
                });

                // Limpiar entradas expiradas (opcional)
                cache.retain(|_, entry| entry.timestamp.elapsed() < cache_duration);
            }

            Json(json!({
                "status": "success",
                "data": users,
                "count": users.len(),
                "limit": limit,
                "cached": false
            }))
        }
        Err(e) => Json(json!({
            "status": "error",
            "message": format!("Error: {}", e)
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL debe estar definida");
    
    // Conectar a PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await?;
    
    // Crear cache local en memoria
    let cache = Arc::new(RwLock::new(HashMap::new()));
    
    let app_state = AppState { 
        db: pool,
        cache,
    };
    
    let app = Router::new()
        .route("/users", get(get_users))
        .nest_service("/", ServeDir::new("public"))
        .with_state(app_state);

    let port = env::var("API_PORT").unwrap_or_else(|_| "80".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    
    axum::serve(listener, app).await?;
    Ok(())
}
