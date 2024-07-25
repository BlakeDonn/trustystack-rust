use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

pub struct Query;

#[juniper::graphql_object]
impl Query {
    fn api_version() -> FieldResult<String> {
        Ok("1.0".to_string())
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}

