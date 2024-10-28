// src/graphql_schema/parts/storage_spec.rs

use crate::graphql_schema::context::Context;
use crate::models::parts::storage_spec::StorageSpec;
use juniper::graphql_object;

/// `StorageSpecGraphQL` struct representing Storage specifications in the GraphQL schema.
pub struct StorageSpecGraphQL {
    pub part_id: i32,
    pub capacity: Option<i32>,
    pub interface: Option<String>,
    pub form_factor: Option<String>,
    pub sequential_read_speed: Option<i32>,
    pub sequential_write_speed: Option<i32>,
    pub nand_type: Option<String>,
    pub controller: Option<String>,
    pub endurance: Option<i32>,
    pub encryption_support: Option<bool>,
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
            nand_type: storage_spec.nand_type,
            controller: storage_spec.controller,
            endurance: storage_spec.endurance,
            encryption_support: storage_spec.encryption_support,
        }
    }
}

#[graphql_object(context = Context)]
impl StorageSpecGraphQL {
    fn part_id(&self) -> i32 {
        self.part_id
    }

    fn capacity(&self) -> Option<i32> {
        self.capacity
    }

    fn interface(&self) -> Option<&str> {
        self.interface.as_deref()
    }

    fn form_factor(&self) -> Option<&str> {
        self.form_factor.as_deref()
    }

    fn sequential_read_speed(&self) -> Option<i32> {
        self.sequential_read_speed
    }

    fn sequential_write_speed(&self) -> Option<i32> {
        self.sequential_write_speed
    }

    fn nand_type(&self) -> Option<&str> {
        self.nand_type.as_deref()
    }

    fn controller(&self) -> Option<&str> {
        self.controller.as_deref()
    }

    fn endurance(&self) -> Option<i32> {
        self.endurance
    }

    fn encryption_support(&self) -> Option<bool> {
        self.encryption_support
    }
}
