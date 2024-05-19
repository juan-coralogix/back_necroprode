-- migrations/<timestamp>_create_schema/up.sql

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE necroprodes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    creator_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creator_id) REFERENCES users(id)
);

CREATE TABLE necroprode_members (
    necroprode_id INTEGER,
    user_id INTEGER,
    PRIMARY KEY (necroprode_id, user_id),
    FOREIGN KEY (necroprode_id) REFERENCES necroprodes(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE selections (
    id SERIAL PRIMARY KEY,
    necroprode_id INTEGER,
    user_id INTEGER,
    celebrity_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (necroprode_id) REFERENCES necroprodes(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);