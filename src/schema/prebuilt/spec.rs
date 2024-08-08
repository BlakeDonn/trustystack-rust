use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Spec {
    pub key: String,
    pub value: String,
}
