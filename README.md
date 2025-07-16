# 🦀 Rust SQL Base

A modern, high-performance REST API built with Rust, featuring PostgreSQL integration, local caching, and Docker containerization. This project demonstrates best practices for building scalable web services with Rust.

## ✨ Features

- **🚀 High Performance**: Built with Rust and Axum for maximum speed and memory safety
- **🗄️ PostgreSQL Integration**: Robust database operations with SQLx and async support
- **💾 Local Caching**: In-memory caching system with TTL for optimal performance
- **🐳 Docker Ready**: Multi-stage Docker build for minimal production images
- **🌐 Static File Serving**: Serves HTML/CSS/JS files with hot-reload support
- **🔧 Environment Configuration**: Flexible configuration via environment variables
- **📊 Modern Web UI**: Beautiful glassmorphism interface for API interaction

## 🏗️ Tech Stack

### Backend
- **Language**: Rust (Edition 2021)
- **Web Framework**: Axum 0.7
- **Database**: PostgreSQL 16
- **ORM**: SQLx 0.7.4 with compile-time checked queries
- **Runtime**: Tokio async runtime
- **Serialization**: Serde for JSON handling

### Infrastructure
- **Containerization**: Docker with multi-stage builds
- **Orchestration**: Docker Compose
- **Base Image**: Scratch (ultra-minimal final image)
- **Static Files**: Tower-HTTP for file serving

## 📁 Project Structure

```
RustSqlBase/
├── backend/
│   ├── src/
│   │   └── main.rs          # Main application code
│   ├── public/
│   │   └── index.html       # Web interface (hot-reload)
│   ├── Cargo.toml           # Rust dependencies
│   └── Dockerfile           # Multi-stage build
├── db/
│   ├── init.sql             # Database initialization
│   └── check_data.sql       # Data verification script
├── docker-compose.yml       # Service orchestration
├── .env                     # Environment configuration
└── README.md               # This file
```

## 🚦 Quick Start

### Prerequisites
- Docker and Docker Compose
- Git

### 1. Clone the Repository
```bash
git clone <your-repo-url>
cd RustSqlBase
```

### 2. Environment Setup
The project includes a pre-configured `.env` file with default values:

```env
# Database Configuration
POSTGRES_DB=rustdb
POSTGRES_USER=rustuser
POSTGRES_PASSWORD=rustpass
POSTGRES_PORT=5432

# API Configuration
API_PORT=80
DATABASE_URL=postgresql://rustuser:rustpass@postgres:5432/rustdb

# Application Settings
JWT_SECRET=your-jwt-secret-here
RUST_LOG=info
ENVIRONMENT=development
```

### 3. Build and Run
```bash
# Build and start all services
docker-compose up --build

# Or run in background
docker-compose up -d --build
```

### 4. Access the Application
- **Web Interface**: http://localhost
- **API Endpoint**: http://localhost/users
- **With Limit**: http://localhost/users?limit=5

## 🔌 API Endpoints

### GET /users
Retrieve a list of users from the database.

**Query Parameters:**
- `limit` (optional): Maximum number of users to return (default: 100, max: 1000)

**Response Format:**
```json
{
  "status": "success",
  "data": [
    {
      "id": 1,
      "name": "Juan Pérez",
      "email": "juan@example.com",
      "created_at": "2025-01-01T12:00:00Z"
    }
  ],
  "count": 3,
  "limit": 100,
  "cached": false
}
```

**Error Response:**
```json
{
  "status": "error",
  "message": "Database connection failed"
}
```

## 🏛️ Database Schema

### Users Table
```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(150) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Sample Data
The database is automatically initialized with sample users:
- Juan Pérez (juan@example.com)
- María García (maria@example.com)  
- Carlos López (carlos@example.com)

## 💾 Caching System

The API implements a local in-memory caching system:

- **TTL**: 5 minutes per cache entry
- **Key Format**: `users_limit_{limit}`
- **Auto Cleanup**: Expired entries are automatically removed
- **Thread Safe**: Uses Arc<RwLock<>> for concurrent access

**Cache Response Indicators:**
- `"cached": true` - Data served from cache
- `"cached": false` - Data fetched from database

## 🐳 Docker Architecture

### Multi-Stage Build
1. **Builder Stage**: Uses `rust:alpine` for compilation with musl target
2. **Final Stage**: Uses `scratch` for ultra-minimal runtime image (< 20MB)

### Container Features
- **Static Binary**: Self-contained executable with no dependencies
- **Hot Reload**: Public folder mounted as volume for development
- **Health Checks**: PostgreSQL container includes health monitoring
- **Automatic Restart**: Services restart on failure

## 🛠️ Development

### Local Development with Hot Reload
```bash
# Start only the database
docker-compose up postgres -d

# Run Rust application locally (requires Rust toolchain)
cd backend
cargo run

# Modify files in backend/public/ - changes appear immediately
```

### Database Management
```bash
# Connect to PostgreSQL
docker-compose exec postgres psql -U rustuser -d rustdb

# Check database contents
docker-compose exec postgres psql -U rustuser -d rustdb -f /docker-entrypoint-initdb.d/check_data.sql

# View logs
docker-compose logs postgres
docker-compose logs rust-app
```

### Building for Production
```bash
# Build optimized image
docker-compose build

# Run with production settings
ENVIRONMENT=production docker-compose up -d
```

## 📊 Monitoring and Debugging

### Application Logs
```bash
# View real-time logs
docker-compose logs -f rust-app

# PostgreSQL logs
docker-compose logs -f postgres
```

### Performance Metrics
- **Cache Hit Rate**: Monitor via API responses (`cached: true/false`)
- **Response Times**: Built-in Rust performance
- **Memory Usage**: Ultra-low due to Rust's zero-cost abstractions

### Debugging Tools
- **Web Console**: Browser developer tools show API interactions
- **Database Queries**: SQLx provides compile-time query validation
- **Container Health**: Docker Compose health checks

## 🚀 Production Deployment

### Optimization Checklist
- [ ] Update JWT secret in `.env`
- [ ] Set `ENVIRONMENT=production`
- [ ] Configure proper database credentials
- [ ] Set up reverse proxy (nginx/traefik)
- [ ] Enable SSL/TLS certificates
- [ ] Configure log aggregation
- [ ] Set up monitoring (Prometheus/Grafana)

### Security Considerations
- Database credentials in environment variables
- JWT tokens for authentication (if implemented)
- Input validation via Rust's type system
- SQL injection prevention with SQLx
- Memory safety guaranteed by Rust

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Rust Community**: For the amazing ecosystem
- **Axum Team**: For the excellent web framework
- **SQLx Team**: For compile-time checked SQL
- **PostgreSQL**: For the robust database engine

---

Made with ❤️ using Rust + Axum + PostgreSQL