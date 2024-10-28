use crate::graphql_schema::context::Context;
use crate::graphql_schema::parts::manufacturer_graphql::ManufacturerGraphQL;
use crate::models::parts::manufacturer::Manufacturer;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};
use log::error;

/// `ManufacturersQueries` struct to encapsulate manufacturer-related queries.
pub struct ManufacturersQueries;

impl ManufacturersQueries {
    /// Fetches all manufacturers from the database.
    pub fn get_all_manufacturers(context: &Context) -> FieldResult<Vec<ManufacturerGraphQL>> {
        use crate::diesel_schema::parts::manufacturers::dsl::*;

        let mut conn = context.get_connection().map_err(|e| {
            error!("Database connection error: {}", e);
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let manufacturers_list = manufacturers.load::<Manufacturer>(&mut conn).map_err(|e| {
            error!("Error fetching manufacturers: {}", e);
            FieldError::new(
                "Error fetching manufacturers",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let graphql_manufacturers = manufacturers_list
            .into_iter()
            .map(ManufacturerGraphQL::from_manufacturer)
            .collect();

        Ok(graphql_manufacturers)
    }

    /// Fetches a specific manufacturer by ID.
    pub fn get_manufacturer_by_id(
        context: &Context,
        manufacturer_id_val: i32,
    ) -> FieldResult<Option<ManufacturerGraphQL>> {
        use crate::diesel_schema::parts::manufacturers::dsl::*;

        let mut conn = context.get_connection().map_err(|e| {
            error!("Database connection error: {}", e);
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let manufacturer_opt = manufacturers
            .filter(id.eq(manufacturer_id_val))
            .first::<Manufacturer>(&mut conn)
            .optional()
            .map_err(|e| {
                error!("Error fetching manufacturer: {}", e);
                FieldError::new(
                    "Error fetching manufacturer",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        match manufacturer_opt {
            Some(manufacturer) => Ok(Some(ManufacturerGraphQL::from_manufacturer(manufacturer))),
            None => Ok(None),
        }
    }
}
