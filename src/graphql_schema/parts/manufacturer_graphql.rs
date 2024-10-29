// src/graphql_schema/parts/manufacturer.rs

use crate::graphql_schema::context::Context;
use crate::models::parts::manufacturer::Manufacturer;
use juniper::graphql_object;

/// `ManufacturerGraphQL` struct representing a manufacturer in the GraphQL schema.
pub struct ManufacturerGraphQL {
    pub id: i32,
    pub name: String,
    pub website: Option<String>,
}

impl ManufacturerGraphQL {
    /// Converts a `Manufacturer` model into a `ManufacturerGraphQL`.
    pub fn from_manufacturer(manufacturer: Manufacturer) -> Self {
        ManufacturerGraphQL {
            id: manufacturer.id,
            name: manufacturer.name,
            website: manufacturer.website,
        }
    }
}

#[graphql_object(context = Context)]
impl ManufacturerGraphQL {
    /// Returns the ID of the manufacturer.
    fn id(&self) -> i32 {
        self.id
    }

    /// Returns the name of the manufacturer.
    fn name(&self) -> &str {
        &self.name
    }

    /// Returns the website of the manufacturer.
    fn website(&self) -> Option<&str> {
        self.website.as_deref()
    }
}
