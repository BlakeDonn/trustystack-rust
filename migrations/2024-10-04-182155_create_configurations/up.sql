CREATE TABLE configurations (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    name VARCHAR NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE configuration_parts (
    configuration_id INTEGER REFERENCES configurations(id),
    part_id INTEGER REFERENCES parts(id),
    quantity INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (configuration_id, part_id)
);

