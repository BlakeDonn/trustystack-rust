use crate::graphql_schema::context::Context;
use crate::graphql_schema::parts::category_graphql::CategoryGraphQL;
use crate::graphql_schema::parts::cpu_spec_graphql::CpuSpecGraphQL;
use crate::graphql_schema::parts::gpu_spec_graphql::GpuSpecGraphQL;
use crate::graphql_schema::parts::manufacturer_graphql::ManufacturerGraphQL;
use crate::graphql_schema::parts::memory_spec_graphql::MemorySpecGraphQL;
use crate::graphql_schema::parts::storage_spec_graphql::StorageSpecGraphQL;
use crate::models::parts::part::Part;
use juniper::{graphql_object, FieldResult};

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

    fn common_specifications(&self) -> Option<&str> {
        self.common_specifications.as_deref()
    }

    fn manufacturer(&self, context: &Context) -> FieldResult<Option<ManufacturerGraphQL>> {
        match self.manufacturer_id {
            Some(id) => {
                let manufacturer = context.get_manufacturer_by_id(id)?;
                Ok(Some(ManufacturerGraphQL::from_manufacturer(manufacturer)))
            }
            None => Ok(None),
        }
    }

    fn category(&self, context: &Context) -> FieldResult<Option<CategoryGraphQL>> {
        match self.category_id {
            Some(id) => {
                let category = context.get_category_by_id(id)?;
                Ok(Some(CategoryGraphQL::from_category(category)))
            }
            None => Ok(None),
        }
    }

    /// Resolves the GPU specifications associated with the part.
    fn gpu_spec(&self, context: &Context) -> FieldResult<Option<GpuSpecGraphQL>> {
        match context.get_gpu_spec_by_part_id(self.id) {
            Ok(spec) => Ok(spec.map(GpuSpecGraphQL::from_gpu_spec)),
            Err(e) => Err(e), // Propagate other errors
        }
    }

    /// Resolves the Memory specifications associated with the part.
    fn memory_spec(&self, context: &Context) -> FieldResult<Option<MemorySpecGraphQL>> {
        match context.get_memory_spec_by_part_id(self.id) {
            Ok(spec) => Ok(spec.map(MemorySpecGraphQL::from_memory_spec)),
            Err(e) => Err(e),
        }
    }

    /// Resolves the Storage specifications associated with the part.
    fn storage_spec(&self, context: &Context) -> FieldResult<Option<StorageSpecGraphQL>> {
        match context.get_storage_spec_by_part_id(self.id) {
            Ok(spec) => Ok(spec.map(StorageSpecGraphQL::from_storage_spec)),
            Err(e) => Err(e),
        }
    }

    /// Resolves the CPU specifications associated with the part.
    fn cpu_spec(&self, context: &Context) -> FieldResult<Option<CpuSpecGraphQL>> {
        match context.get_cpu_spec_by_part_id(self.id) {
            Ok(spec) => Ok(spec.map(CpuSpecGraphQL::from_cpu_spec)),
            Err(e) => Err(e),
        }
    }
}
