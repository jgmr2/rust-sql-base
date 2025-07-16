-- Script para verificar datos en la base
\c rustdb;

-- Mostrar usuarios
SELECT * FROM users;

-- Contar usuarios
SELECT COUNT(*) as total_users FROM users;

-- Describir la tabla
\d users;
