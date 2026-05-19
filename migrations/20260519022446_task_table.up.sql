-- Add up migration script here
CREATE TYPE task_status AS ENUM ('todo', 'doing', 'done');
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    status task_status NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    user_id SERIAL REFERENCES users(id)
);
