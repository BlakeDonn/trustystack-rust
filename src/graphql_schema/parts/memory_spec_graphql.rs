// src/graphql_schema/parts/memory_spec_graphql.rs

use crate::models::parts::memory_spec::MemorySpec;
use juniper::GraphQLObject;

/// `MemorySpecGraphQL` struct representing Memory Specifications in the GraphQL schema.
#[derive(GraphQLObject)]
#[graphql(description = "Memory Specifications")]
pub struct MemorySpecGraphQL {
    pub part_id: i32,
    pub capacity: Option<i32>,
    pub speed: Option<i32>,
    pub memory_type: Option<String>,
    pub ecc: Option<bool>,
}

impl MemorySpecGraphQL {
    /// Converts a `MemorySpec` model into a `MemorySpecGraphQL`.
    pub fn from_memory_spec(memory_spec: MemorySpec) -> Self {
        MemorySpecGraphQL {
            part_id: memory_spec.part_id,
            capacity: memory_spec.capacity,
            speed: memory_spec.speed,
            memory_type: memory_spec.memory_type,
            ecc: memory_spec.ecc,
        }
    }
}
