// src/graphql_schema/parts/memory_spec_graphql.rs

use crate::graphql_schema::context::Context;
use crate::models::parts::memory_spec::MemorySpec;
use juniper::graphql_object;

/// `MemorySpecGraphQL` struct representing Memory specifications in the GraphQL schema.
pub struct MemorySpecGraphQL {
    pub part_id: i32,
    pub capacity: Option<i32>,
    pub speed: Option<i32>,
    pub memory_type: Option<String>,
    pub ecc: Option<bool>,
    pub buffered: Option<bool>,
    pub cas_latency: Option<String>, // Converted BigDecimal to String
    pub form_factor: Option<String>,
    pub rgb_lighting: Option<bool>,
    pub kit_configuration: Option<String>,
    pub voltage: Option<String>, // Converted BigDecimal to String
    pub heat_spreader: Option<bool>,
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
            buffered: memory_spec.buffered,
            cas_latency: memory_spec.cas_latency.map(|cl| cl.to_string()),
            form_factor: memory_spec.form_factor,
            rgb_lighting: memory_spec.rgb_lighting,
            kit_configuration: memory_spec.kit_configuration,
            voltage: memory_spec.voltage.map(|v| v.to_string()),
            heat_spreader: memory_spec.heat_spreader,
        }
    }
}

#[graphql_object(context = Context)]
impl MemorySpecGraphQL {
    fn part_id(&self) -> i32 {
        self.part_id
    }

    fn capacity(&self) -> Option<i32> {
        self.capacity
    }

    fn speed(&self) -> Option<i32> {
        self.speed
    }

    fn memory_type(&self) -> Option<&str> {
        self.memory_type.as_deref()
    }

    fn ecc(&self) -> Option<bool> {
        self.ecc
    }

    fn buffered(&self) -> Option<bool> {
        self.buffered
    }

    fn cas_latency(&self) -> Option<&str> {
        self.cas_latency.as_deref()
    }

    fn form_factor(&self) -> Option<&str> {
        self.form_factor.as_deref()
    }

    fn rgb_lighting(&self) -> Option<bool> {
        self.rgb_lighting
    }

    fn kit_configuration(&self) -> Option<&str> {
        self.kit_configuration.as_deref()
    }

    fn voltage(&self) -> Option<&str> {
        self.voltage.as_deref()
    }

    fn heat_spreader(&self) -> Option<bool> {
        self.heat_spreader
    }
}
