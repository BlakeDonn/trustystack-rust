// src/graphql_schema/parts/category.rs

use crate::graphql_schema::context::Context;
use crate::models::parts::category::Category;
use juniper::graphql_object;

/// `CategoryGraphQL` struct representing a category in the GraphQL schema.
pub struct CategoryGraphQL {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl CategoryGraphQL {
    /// Converts a `Category` model into a `CategoryGraphQL`.
    pub fn from_category(category: Category) -> Self {
        CategoryGraphQL {
            id: category.id,
            name: category.name,
            description: category.description,
        }
    }
}

#[graphql_object(context = Context)]
impl CategoryGraphQL {
    /// Returns the ID of the category.
    fn id(&self) -> i32 {
        self.id
    }

    /// Returns the name of the category.
    fn name(&self) -> &str {
        &self.name
    }

    /// Returns the description of the category.
    fn description(&self) -> &str {
        &self.name
    }
}
