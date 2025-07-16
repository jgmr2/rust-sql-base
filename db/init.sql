-- Inicialización de la base de datos
-- Este archivo se ejecuta automáticamente al crear el contenedor

-- Crear tabla de usuarios
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(150) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Crear tabla de posts (ejemplo)
CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    content TEXT,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Insertar datos de prueba
INSERT INTO users (name, email, password_hash) VALUES 
    ('Juan Pérez', 'juan@example.com', '$2b$12$dummy.hash.for.testing'),
    ('María García', 'maria@example.com', '$2b$12$dummy.hash.for.testing'),
    ('Carlos López', 'carlos@example.com', '$2b$12$dummy.hash.for.testing');

INSERT INTO posts (title, content, user_id) VALUES 
    ('Mi primer post', 'Este es el contenido de mi primer post en Rust!', 1),
    ('Aprendiendo PostgreSQL', 'PostgreSQL es genial para aplicaciones Rust', 2),
    ('Docker y Rust', 'Combinando Docker con Rust para desarrollo', 1);

-- Crear índices para optimizar consultas
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_posts_user_id ON posts(user_id);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);

-- Función para actualizar timestamp automáticamente
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers para actualizar updated_at automáticamente
CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_posts_updated_at 
    BEFORE UPDATE ON posts 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
