// src/graphql_schemuse crate::graphql_schema::context::Context;
use crate::graphql_schema::parts::category_graphql::CategoryGraphQL;
use crate::graphql_schema::parts::manufacturer_graphql::ManufacturerGraphQL;
use crate::graphql_schema::parts::part_graphql::PartGraphQL;
use crate::graphql_schema::queries::categories_queries::CategoriesQueries;
use crate::graphql_schema::queries::manufacturers_queries::ManufacturersQueries;
use crate::graphql_schema::queries::parts_queries::{get_all_parts, get_part_by_id};
use juniper::{EmptyMutation, EmptySubscription, RootNode};

/// RootQuery struct that defines the available GraphQL queries in the API.
pub struct RootQuery;

#[juniper::graphql_object(context = Context)]
impl RootQuery {
    /// Returns the current API version.
    fn api_version() -> &str {
        "1.0"
    }

    /// Fetches all parts from the database with optional pagination.
    fn parts(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> juniper::FieldResult<Vec<PartGraphQL>> {
        get_all_parts(context, limit, offset)
    }

    /// Fetches a specific part by ID.
    fn part(context: &Context, part_id: i32) -> juniper::FieldResult<Option<PartGraphQL>> {
        get_part_by_id(context, part_id)
    }

    /// Fetches all manufacturers.
    fn manufacturers(context: &Context) -> juniper::FieldResult<Vec<ManufacturerGraphQL>> {
        ManufacturersQueries::get_all_manufacturers(context)
    }

    /// Fetches a specific manufacturer by ID.
    fn manufacturer(
        context: &Context,
        manufacturer_id: i32,
    ) -> juniper::FieldResult<Option<ManufacturerGraphQL>> {
        ManufacturersQueries::get_manufacturer_by_id(context, manufacturer_id)
    }

    /// Fetches all categories.
    fn categories(context: &Context) -> juniper::FieldResult<Vec<CategoryGraphQL>> {
        CategoriesQueries::get_all_categories(context)
    }

    /// Fetches a specific category by ID.
    fn category(
        context: &Context,
        category_id: i32,
    ) -> juniper::FieldResult<Option<CategoryGraphQL>> {
        CategoriesQueries::get_category_by_id(context, category_id)
    }
}

// Define the Schema
pub type SchemaType = RootNode<'static, RootQuery, EmptyMutation, EmptySubscription>;

/// Creates the GraphQL schema.
pub fn create_schema() -> SchemaType {
    SchemaType::new(RootQuery, EmptyMutation::new(), EmptySubscription::new())
}

use super::context::Context;
