use super::context::Context;
use super::root_query::RootQuery;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

/// The main GraphQL schema type for the application.
pub type Schema = RootNode<'static, RootQuery, EmptyMutation<Context>, EmptySubscription<Context>>;

/// Creates and returns the main GraphQL schema.
pub fn create_schema() -> Schema {
    Schema::new(RootQuery, EmptyMutation::new(), EmptySubscription::new())
}
