-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);
