pub mod context;
pub mod parts;
pub mod prebuilt;
pub mod root_query;
pub mod service;
pub mod software;

use context::Context;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use root_query::RootQuery;

/// The main GraphQL schema type for the application.
pub type Schema = RootNode<'static, RootQuery, EmptyMutation<Context>, EmptySubscription<Context>>;

/// Creates and returns the main GraphQL schema.
pub fn create_schema() -> Schema {
    Schema::new(RootQuery, EmptyMutation::new(), EmptySubscription::new())
}

