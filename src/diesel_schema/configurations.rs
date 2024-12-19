// @generated automatically by Diesel CLI.

diesel::table! {
    configuration_parts (configuration_id, part_id) {
        configuration_id -> Int4,
        part_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    configurations (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(configuration_parts -> configurations (configuration_id));

diesel::allow_tables_to_appear_in_same_query!(
    configuration_parts,
    configurations,
);
