// src/graphql_schema/parts/part_graphql.rs

use crate::graphql_schema::context::Context;
use crate::graphql_schema::parts::category_graphql::CategoryGraphQL;
use crate::graphql_schema::parts::cpu_spec_graphql::CpuSpecGraphQL;
use crate::graphql_schema::parts::gpu_spec_graphql::GpuSpecGraphQL;
use crate::graphql_schema::parts::manufacturer_graphql::ManufacturerGraphQL;
use crate::graphql_schema::parts::memory_spec_graphql::MemorySpecGraphQL;
use crate::graphql_schema::parts::storage_spec_graphql::StorageSpecGraphQL;
use crate::models::parts::part::Part;
use juniper::{graphql_object, FieldResult};
use log::{error, info};

/// `PartGraphQL` struct representing a PC Part in the GraphQL schema.
pub struct PartGraphQL {
    pub id: i32,
    pub manufacturer_id: Option<i32>,
    pub category_id: Option<i32>,
    pub name: String,
    pub model: String,
    pub price: Option<String>,
    pub common_specifications: Option<String>,
}

impl PartGraphQL {
    /// Converts a `Part` model into a `PartGraphQL`.
    pub fn from_part(part: Part) -> Self {
        PartGraphQL {
            id: part.id,
            manufacturer_id: part.manufacturer_id,
            category_id: part.category_id,
            name: part.name,
            model: part.model,
            price: part.price.map(|p| p.to_string()),
            common_specifications: part.common_specifications.map(|spec| spec.to_string()),
        }
    }
}

#[graphql_object(context = Context)]
impl PartGraphQL {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn model(&self) -> &str {
        &self.model
    }

    fn price(&self) -> Option<&str> {
        self.price.as_deref()
    }

    fn commonSpecifications(&self) -> Option<&str> {
        self.common_specifications.as_deref()
    }

    fn manufacturer(&self, context: &Context) -> FieldResult<Option<ManufacturerGraphQL>> {
        info!("Resolving manufacturer for part ID: {}", self.id);
        match self.manufacturer_id {
            Some(id) => {
                let manufacturer = context.get_manufacturer_by_id(id)?;
                info!("Manufacturer resolved: {:?}", manufacturer);
                Ok(Some(ManufacturerGraphQL::from_manufacturer(manufacturer)))
            }
            None => {
                info!("No manufacturer associated with part ID: {}", self.id);
                Ok(None)
            }
        }
    }

    fn category(&self, context: &Context) -> FieldResult<Option<CategoryGraphQL>> {
        info!("Resolving category for part ID: {}", self.id);
        match self.category_id {
            Some(id) => {
                let category = context.get_category_by_id(id)?;
                info!("Category resolved: {:?}", category);
                Ok(Some(CategoryGraphQL::from_category(category)))
            }
            None => {
                info!("No category associated with part ID: {}", self.id);
                Ok(None)
            }
        }
    }

    /// Resolves the GPU specifications associated with the part.
    fn gpuSpec(&self, context: &Context) -> FieldResult<Option<GpuSpecGraphQL>> {
        info!("Resolving GPU spec for part ID: {}", self.id);
        match context.get_gpu_spec_by_part_id(self.id) {
            Ok(spec) => {
                if let Some(ref gpu_spec) = spec {
                    info!("GPU spec found: {:?}", gpu_spec);
                } else {
                    info!("No GPU spec found for part ID: {}", self.id);
                }
                Ok(spec.map(GpuSpecGraphQL::from_gpu_spec))
            }
            Err(e) => {
                error!("Error resolving GPU spec for part ID {}: {:#?}", self.id, e);
                Err(e)
            }
        }
    }

    /// Resolves the Memory specifications associated with the part.
    fn memorySpec(&self, context: &Context) -> FieldResult<Option<MemorySpecGraphQL>> {
        info!("Resolving Memory spec for part ID: {}", self.id);
        match context.get_memory_spec_by_part_id(self.id) {
            Ok(spec) => {
                if let Some(ref memory_spec) = spec {
                    info!("Memory spec found: {:?}", memory_spec);
                } else {
                    info!("No Memory spec found for part ID: {}", self.id);
                }
                Ok(spec.map(MemorySpecGraphQL::from_memory_spec))
            }
            Err(e) => {
                error!(
                    "Error resolving Memory spec for part ID {}: {:#?}",
                    self.id, e
                );
                Err(e)
            }
        }
    }

    /// Resolves the Storage specifications associated with the part.
    fn storageSpec(&self, context: &Context) -> FieldResult<Option<StorageSpecGraphQL>> {
        info!("Resolving Storage spec for part ID: {}", self.id);
        match context.get_storage_spec_by_part_id(self.id) {
            Ok(spec) => {
                if let Some(ref storage_spec) = spec {
                    info!("Storage spec found: {:?}", storage_spec);
                } else {
                    info!("No Storage spec found for part ID: {}", self.id);
                }
                Ok(spec.map(StorageSpecGraphQL::from_storage_spec))
            }
            Err(e) => {
                error!(
                    "Error resolving Storage spec for part ID {}: {:#?}",
                    self.id, e
                );
                Err(e)
            }
        }
    }

    /// Resolves the CPU specifications associated with the part.
    fn cpuSpec(&self, context: &Context) -> FieldResult<Option<CpuSpecGraphQL>> {
        info!("Resolving CPU spec for part ID: {}", self.id);
        match context.get_cpu_spec_by_part_id(self.id) {
            Ok(spec) => {
                if let Some(ref cpu_spec) = spec {
                    info!("CPU spec found: {:?}", cpu_spec);
                } else {
                    info!("No CPU spec found for part ID: {}", self.id);
                }
                Ok(spec.map(CpuSpecGraphQL::from_cpu_spec))
            }
            Err(e) => {
                error!("Error resolving CPU spec for part ID {}: {:#?}", self.id, e);
                Err(e)
            }
        }
    }
}
