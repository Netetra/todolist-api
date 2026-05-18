-- Add up migration script here
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);
