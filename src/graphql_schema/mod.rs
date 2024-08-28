pub mod prebuilt;
pub mod service;
pub mod software;

use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub struct Query;

#[juniper::graphql_object]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }

    fn popular_prebuilts() -> Vec<prebuilt::Prebuilt> {
        prebuilt::get_prebuilts()
    }

    fn services() -> Vec<service::Service> {
        service::get_services()
    }

    fn software_solutions() -> Vec<software::Software> {
        software::get_softwares()
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
