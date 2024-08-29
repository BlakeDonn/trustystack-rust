pub mod context;
pub mod parts;
pub mod prebuilt;
pub mod root_query;
pub mod service;
pub mod software;

use context::Context;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use root_query::RootQuery;

pub type Schema = RootNode<'static, RootQuery, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(RootQuery, EmptyMutation::new(), EmptySubscription::new())
}

