use crate::models::parts::category::Category;
use crate::models::parts::cpu_spec::CpuSpec;
use crate::models::parts::gpu_spec::GpuSpec;
use crate::models::parts::manufacturer::Manufacturer;
use crate::models::parts::memory_spec::MemorySpec;
use crate::models::parts::storage_spec::StorageSpec;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use juniper::{FieldError, FieldResult};

/// Represents the context that holds the database connection pool.
pub struct Context {
    pub db: Pool<ConnectionManager<PgConnection>>,
}

impl Context {
    /// Creates a new context with the provided database connection pool.
    pub fn new(db: Pool<ConnectionManager<PgConnection>>) -> Self {
        log::info!("Creating new Context with database pool.");
        Context { db }
    }

    /// Retrieves a connection from the pool.
    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::r2d2::PoolError> {
        log::info!("Getting a database connection from the pool.");
        self.db.get()
    }

    /// Fetches a manufacturer by ID from the database.
    pub fn get_manufacturer_by_id(&self, manufacturer_id_val: i32) -> FieldResult<Manufacturer> {
        use crate::diesel_schema::parts::manufacturers::dsl::*;

        let mut conn = self.get_connection().map_err(|e| {
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let manufacturer = manufacturers
            .filter(id.eq(manufacturer_id_val))
            .first::<Manufacturer>(&mut conn)
            .map_err(|e| {
                log::error!("Error fetching manufacturer: {}", e);
                FieldError::new(
                    "Error fetching manufacturer",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        Ok(manufacturer)
    }

    /// Fetches a category by ID from the database.
    pub fn get_category_by_id(&self, category_id_val: i32) -> FieldResult<Category> {
        use crate::diesel_schema::parts::categories::dsl::*;

        let mut conn = self.get_connection().map_err(|e| {
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let category = categories
            .filter(id.eq(category_id_val))
            .first::<Category>(&mut conn)
            .map_err(|e| {
                log::error!("Error fetching category: {}", e);
                FieldError::new(
                    "Error fetching category",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        Ok(category)
    }

    /// Fetches a GPU specification by part ID from the database.
    pub fn get_gpu_spec_by_part_id(&self, part_id_val: i32) -> FieldResult<Option<GpuSpec>> {
        use crate::diesel_schema::parts::gpu_specs::dsl::*;

        let mut conn = self.get_connection().map_err(|e| {
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let gpu_spec = gpu_specs
            .filter(part_id.eq(part_id_val))
            .select(GpuSpec::as_select()) // Utilize Selectable
            .first::<GpuSpec>(&mut conn)
            .optional()
            .map_err(|e| {
                log::error!("Error fetching GPU spec: {}", e);
                FieldError::new(
                    "Error fetching GPU spec",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        Ok(gpu_spec)
    }

    /// Fetches a Memory specification by part ID from the database.
    pub fn get_memory_spec_by_part_id(&self, part_id_val: i32) -> FieldResult<Option<MemorySpec>> {
        use crate::diesel_schema::parts::memory_specs::dsl::*;

        let mut conn = self.get_connection().map_err(|e| {
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let memory_spec = memory_specs
            .filter(part_id.eq(part_id_val))
            .select(MemorySpec::as_select()) // Utilize Selectable
            .first::<MemorySpec>(&mut conn)
            .optional()
            .map_err(|e| {
                log::error!("Error fetching Memory spec: {}", e);
                FieldError::new(
                    "Error fetching Memory spec",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        Ok(memory_spec)
    }

    /// Fetches a Storage specification by part ID from the database.
    pub fn get_storage_spec_by_part_id(
        &self,
        part_id_val: i32,
    ) -> FieldResult<Option<StorageSpec>> {
        use crate::diesel_schema::parts::storage_specs::dsl::*;

        let mut conn = self.get_connection().map_err(|e| {
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let storage_spec = storage_specs
            .filter(part_id.eq(part_id_val))
            .select(StorageSpec::as_select())
            .first::<StorageSpec>(&mut conn)
            .optional()
            .map_err(|e| {
                log::error!("Error fetching Storage spec: {}", e);
                FieldError::new(
                    "Error fetching Storage spec",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        Ok(storage_spec)
    }

    /// Fetches a CPU specification by part ID from the database.
    pub fn get_cpu_spec_by_part_id(&self, part_id_val: i32) -> FieldResult<Option<CpuSpec>> {
        use crate::diesel_schema::parts::cpu_specs::dsl::*;

        let mut conn = self.get_connection().map_err(|e| {
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let cpu_spec = cpu_specs
            .filter(part_id.eq(part_id_val))
            .first::<CpuSpec>(&mut conn)
            .optional()
            .map_err(|e| {
                log::error!("Error fetching CPU spec: {}", e);
                FieldError::new(
                    "Error fetching CPU spec",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        Ok(cpu_spec)
    }
}

/// Required to implement Juniper's `Context` trait for integration with GraphQL.
impl juniper::Context for Context {}
