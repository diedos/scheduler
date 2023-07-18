CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    completed_at TIMESTAMP,
    deadline_at TIMESTAMP,
    estimate SMALLINT,
    title TEXT NOT NULL,
    content TEXT
);