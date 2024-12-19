// src/graphql_schema/root_query.rs

use crate::graphql_schema::context::Context;
use crate::graphql_schema::parts::category_graphql::CategoryGraphQL;
use crate::graphql_schema::parts::manufacturer_graphql::ManufacturerGraphQL;
use crate::graphql_schema::parts::part_graphql::PartGraphQL;
use crate::graphql_schema::queries::categories_queries::CategoriesQueries;
use crate::graphql_schema::queries::manufacturers_queries::ManufacturersQueries;
use crate::graphql_schema::queries::parts_queries::{get_all_parts, get_part_by_id};
use crate::graphql_schema::users::query::UserQuery;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use log::{error, info};
use std::time::Instant;

/// RootQuery struct that defines the available GraphQL queries in the API.
pub struct RootQuery;

#[juniper::graphql_object(context = Context)]
impl RootQuery {
    /// Returns the current API version.
    fn apiVersion() -> &str {
        info!("apiVersion query called");
        "1.0"
    }

    /// Fetches all parts from the database with optional pagination.
    fn parts(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> juniper::FieldResult<Vec<PartGraphQL>> {
        let start_time = Instant::now();
        info!(
            "Executing 'parts' query with limit: {:?}, offset: {:?}",
            limit, offset
        );
        let result = get_all_parts(context, limit, offset);
        let duration = start_time.elapsed();

        match &result {
            Ok(parts) => info!("Fetched {} parts in {:?}", parts.len(), duration),
            Err(e) => error!(
                "Error executing 'parts' query: {:?}. Duration: {:?}",
                e, duration
            ),
        }
        result
    }

    /// Fetches a specific part by ID.
    fn part(context: &Context, part_id: i32) -> juniper::FieldResult<Option<PartGraphQL>> {
        let start_time = Instant::now();
        info!("Executing 'part' query with part_id: {}", part_id);
        let result = get_part_by_id(context, part_id);
        let duration = start_time.elapsed();

        match &result {
            Ok(Some(_)) => info!("Part found for part_id: {} in {:?}", part_id, duration),
            Ok(None) => info!(
                "No part found for part_id: {}. Query executed in {:?}",
                part_id, duration
            ),
            Err(e) => error!(
                "Error executing 'part' query: {:?}. Duration: {:?}",
                e, duration
            ),
        }
        result
    }

    /// Fetches all manufacturers.
    fn manufacturers(context: &Context) -> juniper::FieldResult<Vec<ManufacturerGraphQL>> {
        let start_time = Instant::now();
        info!("Executing 'manufacturers' query");
        let result = ManufacturersQueries::get_all_manufacturers(context);
        let duration = start_time.elapsed();

        match &result {
            Ok(manufacturers) => info!(
                "Fetched {} manufacturers in {:?}",
                manufacturers.len(),
                duration
            ),
            Err(e) => error!(
                "Error executing 'manufacturers' query: {:?}. Duration: {:?}",
                e, duration
            ),
        }
        result
    }

    /// Fetches a specific manufacturer by ID.
    fn manufacturer(
        context: &Context,
        manufacturer_id: i32,
    ) -> juniper::FieldResult<Option<ManufacturerGraphQL>> {
        let start_time = Instant::now();
        info!(
            "Executing 'manufacturer' query with manufacturer_id: {}",
            manufacturer_id
        );
        let result = ManufacturersQueries::get_manufacturer_by_id(context, manufacturer_id);
        let duration = start_time.elapsed();

        match &result {
            Ok(Some(_)) => info!(
                "Manufacturer found for manufacturer_id: {} in {:?}",
                manufacturer_id, duration
            ),
            Ok(None) => info!(
                "No manufacturer found for manufacturer_id: {}. Query executed in {:?}",
                manufacturer_id, duration
            ),
            Err(e) => error!(
                "Error executing 'manufacturer' query: {:?}. Duration: {:?}",
                e, duration
            ),
        }
        result
    }

    /// Fetches all categories.
    fn categories(context: &Context) -> juniper::FieldResult<Vec<CategoryGraphQL>> {
        let start_time = Instant::now();
        info!("Executing 'categories' query");
        let result = CategoriesQueries::get_all_categories(context);
        let duration = start_time.elapsed();

        match &result {
            Ok(categories) => info!("Fetched {} categories in {:?}", categories.len(), duration),
            Err(e) => error!(
                "Error executing 'categories' query: {:?}. Duration: {:?}",
                e, duration
            ),
        }
        result
    }

    /// Fetches a specific category by ID.
    fn category(
        context: &Context,
        category_id: i32,
    ) -> juniper::FieldResult<Option<CategoryGraphQL>> {
        let start_time = Instant::now();
        info!(
            "Executing 'category' query with category_id: {}",
            category_id
        );
        let result = CategoriesQueries::get_category_by_id(context, category_id);
        let duration = start_time.elapsed();

        match &result {
            Ok(Some(_)) => info!(
                "Category found for category_id: {} in {:?}",
                category_id, duration
            ),
            Ok(None) => info!(
                "No category found for category_id: {}. Query executed in {:?}",
                category_id, duration
            ),
            Err(e) => error!(
                "Error executing 'category' query: {:?}. Duration: {:?}",
                e, duration
            ),
        }
        result
    }

    fn user_query() -> UserQuery {
        UserQuery
    }
}

// Define the Schema
pub type SchemaType = RootNode<'static, RootQuery, EmptyMutation, EmptySubscription>;

/// Creates the GraphQL schema.
pub fn create_schema() -> SchemaType {
    SchemaType::new(RootQuery, EmptyMutation::new(), EmptySubscription::new())
}
