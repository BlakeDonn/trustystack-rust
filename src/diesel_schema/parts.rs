// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    configuration_parts (configuration_id, part_id) {
        configuration_id -> Int4,
        part_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    cpu_specs (part_id) {
        part_id -> Int4,
        cores -> Nullable<Int4>,
        threads -> Nullable<Int4>,
        base_clock_speed -> Nullable<Numeric>,
        max_boost_clock_speed -> Nullable<Numeric>,
        tdp -> Nullable<Int4>,
        socket_type -> Nullable<Varchar>,
        cache_size -> Nullable<Numeric>,
        integrated_graphics -> Nullable<Bool>,
        process_technology -> Nullable<Numeric>,
    }
}

diesel::table! {
    gpu_specs (part_id) {
        part_id -> Int4,
        cuda_cores -> Nullable<Int4>,
        vram_size -> Nullable<Numeric>,
        vram_type -> Nullable<Varchar>,
        tdp -> Nullable<Int4>,
        memory_bandwidth -> Nullable<Numeric>,
        interface -> Nullable<Varchar>,
        form_factor -> Nullable<Varchar>,
        outputs -> Nullable<Array<Nullable<Text>>>,
        length -> Nullable<Int4>,
    }
}

diesel::table! {
    manufacturers (id) {
        id -> Int4,
        name -> Varchar,
        website -> Nullable<Varchar>,
    }
}

diesel::table! {
    memory_specs (part_id) {
        part_id -> Int4,
        capacity -> Nullable<Int4>,
        speed -> Nullable<Int4>,
        memory_type -> Nullable<Varchar>,
        ecc -> Nullable<Bool>,
        buffered -> Nullable<Bool>,
        cas_latency -> Nullable<Numeric>,
        form_factor -> Nullable<Varchar>,
        rgb_lighting -> Nullable<Bool>,
        kit_configuration -> Nullable<Varchar>,
        voltage -> Nullable<Numeric>,
        heat_spreader -> Nullable<Bool>,
    }
}

diesel::table! {
    parts (id) {
        id -> Int4,
        manufacturer_id -> Nullable<Int4>,
        category_id -> Nullable<Int4>,
        name -> Varchar,
        model -> Varchar,
        price -> Nullable<Numeric>,
        common_specifications -> Nullable<Jsonb>,
    }
}

diesel::table! {
    storage_specs (part_id) {
        part_id -> Int4,
        capacity -> Nullable<Int4>,
        interface -> Nullable<Varchar>,
        form_factor -> Nullable<Varchar>,
        sequential_read_speed -> Nullable<Int4>,
        sequential_write_speed -> Nullable<Int4>,
        nand_type -> Nullable<Varchar>,
        controller -> Nullable<Varchar>,
        endurance -> Nullable<Int4>,
        encryption_support -> Nullable<Bool>,
    }
}

diesel::joinable!(configuration_parts -> parts (part_id));
diesel::joinable!(cpu_specs -> parts (part_id));
diesel::joinable!(gpu_specs -> parts (part_id));
diesel::joinable!(memory_specs -> parts (part_id));
diesel::joinable!(parts -> categories (category_id));
diesel::joinable!(parts -> manufacturers (manufacturer_id));
diesel::joinable!(storage_specs -> parts (part_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    configuration_parts,
    cpu_specs,
    gpu_specs,
    manufacturers,
    memory_specs,
    parts,
    storage_specs,
);
