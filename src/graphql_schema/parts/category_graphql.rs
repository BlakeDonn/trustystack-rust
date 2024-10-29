// src/graphql_schema/parts/category_graphql.rs

use crate::models::parts::category::Category;
use juniper::GraphQLObject;

/// `CategoryGraphQL` struct representing Categories in the GraphQL schema.
#[derive(GraphQLObject)]
#[graphql(description = "Category of Parts")]
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
