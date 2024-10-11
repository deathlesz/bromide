DROP TABLE IF EXISTS accounts CASCADE;

CREATE TABLE accounts (
    id BIGSERIAL PRIMARY KEY,
    user_name VARCHAR(20) UNIQUE NOT NULL,
    password VARCHAR(225) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL
);