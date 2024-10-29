// src/graphql_schema/parts/manufacturer_graphql.rs

use crate::models::parts::manufacturer::Manufacturer;
use juniper::GraphQLObject;

/// `ManufacturerGraphQL` struct representing Manufacturers in the GraphQL schema.
#[derive(GraphQLObject)]
#[graphql(description = "Manufacturer of Parts")]
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
