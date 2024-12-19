use chrono::{DateTime, Utc};
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "A user in the system")]
pub struct UserType {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
}

impl From<crate::models::auth::User> for UserType {
    fn from(user: crate::models::auth::User) -> Self {
        UserType {
            id: user.id,
            name: user.name,
            email: user.email,
            email_verified: user.email_verified,
            image: user.image,
        }
    }
}
