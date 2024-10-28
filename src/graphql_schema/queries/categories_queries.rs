use crate::graphql_schema::context::Context;
use crate::graphql_schema::parts::category_graphql::CategoryGraphQL;
use crate::models::parts::category::Category;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};
use log::error;

/// `CategoriesQueries` struct to encapsulate category-related queries.
pub struct CategoriesQueries;

impl CategoriesQueries {
    /// Fetches all categories from the database.
    pub fn get_all_categories(context: &Context) -> FieldResult<Vec<CategoryGraphQL>> {
        use crate::diesel_schema::parts::categories::dsl::*;

        let mut conn = context.get_connection().map_err(|e| {
            error!("Database connection error: {}", e);
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let categories_list = categories.load::<Category>(&mut conn).map_err(|e| {
            error!("Error fetching categories: {}", e);
            FieldError::new(
                "Error fetching categories",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let graphql_categories = categories_list
            .into_iter()
            .map(CategoryGraphQL::from_category)
            .collect();

        Ok(graphql_categories)
    }

    /// Fetches a specific category by ID.
    pub fn get_category_by_id(
        context: &Context,
        category_id_val: i32,
    ) -> FieldResult<Option<CategoryGraphQL>> {
        use crate::diesel_schema::parts::categories::dsl::*;

        let mut conn = context.get_connection().map_err(|e| {
            error!("Database connection error: {}", e);
            FieldError::new(
                "Database connection error",
                juniper::Value::scalar(e.to_string()),
            )
        })?;

        let category_opt = categories
            .filter(id.eq(category_id_val))
            .first::<Category>(&mut conn)
            .optional()
            .map_err(|e| {
                error!("Error fetching category: {}", e);
                FieldError::new(
                    "Error fetching category",
                    juniper::Value::scalar(e.to_string()),
                )
            })?;

        match category_opt {
            Some(category) => Ok(Some(CategoryGraphQL::from_category(category))),
            None => Ok(None),
        }
    }
}
