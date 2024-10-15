CREATE TABLE parts (
    id SERIAL PRIMARY KEY,
    manufacturer_id INTEGER REFERENCES manufacturers(id),
    category_id INTEGER REFERENCES categories(id),
    name VARCHAR NOT NULL,
    model VARCHAR NOT NULL,
    price NUMERIC,
    common_specifications JSONB
);

