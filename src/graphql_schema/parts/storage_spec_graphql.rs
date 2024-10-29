// src/graphql_schema/parts/storage_spec_graphql.rs

use crate::models::parts::storage_spec::StorageSpec;
use juniper::GraphQLObject;

/// `StorageSpecGraphQL` struct representing Storage Specifications in the GraphQL schema.
#[derive(GraphQLObject)]
#[graphql(description = "Storage Specifications")]
pub struct StorageSpecGraphQL {
    pub part_id: i32,
    pub capacity: Option<i32>,
    pub interface: Option<String>,
    pub form_factor: Option<String>,
    pub sequential_read_speed: Option<i32>,
    pub sequential_write_speed: Option<i32>,
}

impl StorageSpecGraphQL {
    /// Converts a `StorageSpec` model into a `StorageSpecGraphQL`.
    pub fn from_storage_spec(storage_spec: StorageSpec) -> Self {
        StorageSpecGraphQL {
            part_id: storage_spec.part_id,
            capacity: storage_spec.capacity,
            interface: storage_spec.interface,
            form_factor: storage_spec.form_factor,
            sequential_read_speed: storage_spec.sequential_read_speed,
            sequential_write_speed: storage_spec.sequential_write_speed,
        }
    }
}
