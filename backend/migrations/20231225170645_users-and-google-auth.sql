CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    display_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL
);

CREATE TYPE identity_provider AS ENUM ('google');

CREATE TABLE IF NOT EXISTS users_idp (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    user_id INTEGER NOT NULL REFERENCES users(id),
    provider identity_provider NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL
);