CREATE TABLE users (
    id SERIAL PRIMARY KEY,  
    github_id INT NOT NULL UNIQUE,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    avatar_url VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE sessions (
    id VARCHAR(255) PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id),
    expires_at TIMESTAMPTZ NOT NULL
);
