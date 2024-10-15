-- CPU Specifications
CREATE TABLE cpu_specs (
    part_id INTEGER PRIMARY KEY REFERENCES parts(id),
    cores INTEGER,
    threads INTEGER,
    base_clock_speed NUMERIC,
    max_boost_clock_speed NUMERIC,
    tdp INTEGER,
    socket_type VARCHAR,
    cache_size NUMERIC,
    integrated_graphics BOOLEAN,
    process_technology NUMERIC
);

-- GPU Specifications
CREATE TABLE gpu_specs (
    part_id INTEGER PRIMARY KEY REFERENCES parts(id),
    cuda_cores INTEGER,
    vram_size NUMERIC,
    vram_type VARCHAR,
    tdp INTEGER,
    memory_bandwidth NUMERIC,
    interface VARCHAR,
    form_factor VARCHAR,
    outputs TEXT[],
    length INTEGER
);

-- Memory Specifications
CREATE TABLE memory_specs (
    part_id INTEGER PRIMARY KEY REFERENCES parts(id),
    capacity INTEGER,
    speed INTEGER,
    memory_type VARCHAR,
    ecc BOOLEAN,
    buffered BOOLEAN,
    cas_latency NUMERIC,
    form_factor VARCHAR,
    rgb_lighting BOOLEAN,
    kit_configuration VARCHAR,
    voltage NUMERIC,
    heat_spreader BOOLEAN
);

-- Storage Specifications
CREATE TABLE storage_specs (
    part_id INTEGER PRIMARY KEY REFERENCES parts(id),
    capacity INTEGER,
    interface VARCHAR,
    form_factor VARCHAR,
    sequential_read_speed INTEGER,
    sequential_write_speed INTEGER,
    nand_type VARCHAR,
    controller VARCHAR,
    endurance INTEGER,
    encryption_support BOOLEAN
);

