CREATE TABLE manufacturers (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    website VARCHAR
);

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    description TEXT
);
