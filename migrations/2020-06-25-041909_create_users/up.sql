CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO users(name) VALUES ('user1'), ('user2'), ('user3');
