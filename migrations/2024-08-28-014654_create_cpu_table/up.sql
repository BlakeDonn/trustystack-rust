CREATE TABLE cpu (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    core_count INTEGER NOT NULL,
    core_clock VARCHAR(50) NOT NULL,
    boost_clock VARCHAR(50) NOT NULL,
    tdp INTEGER NOT NULL,
    integrated_graphics VARCHAR(100),
    smt BOOLEAN NOT NULL
);
