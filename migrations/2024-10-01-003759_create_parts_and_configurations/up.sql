-- Create the parts table
CREATE TABLE parts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    type TEXT CHECK (type IN ('CPU', 'GPU', 'Motherboard', 'Memory', 'Storage')) NOT NULL,
    specifications JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create the configurations table
CREATE TABLE configurations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT,
    user_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create the configuration_parts table for the many-to-many relationship
CREATE TABLE configuration_parts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    configuration_id UUID REFERENCES configurations(id) ON DELETE CASCADE,
    part_id UUID REFERENCES parts(id) ON DELETE CASCADE,
    quantity INT DEFAULT 1 NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create the performance_benchmarks table
CREATE TABLE performance_benchmarks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    configuration_id UUID REFERENCES configurations(id) ON DELETE CASCADE,
    test_name TEXT NOT NULL,
    result JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

