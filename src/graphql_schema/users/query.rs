use crate::graphql_schema::context::Context;
use crate::graphql_schema::users::types::UserType;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

pub struct UserQuery;

#[juniper::graphql_object(Context = Context)]
impl UserQuery {
    fn user(context: &Context, user_id: i32) -> FieldResult<UserType> {
        use crate::diesel_schema::users::users::dsl::*;

        let conn = &mut context.get_connection()?;
        let user = users
            .find(user_id)
            .first::<crate::models::auth::User>(conn)
            .map_err(|e| FieldError::from(e))?;

        Ok(UserType::from(user))
    }

    fn users(context: &Context) -> FieldResult<Vec<UserType>> {
        use crate::diesel_schema::users::users::dsl::*;

        let conn = &mut context.get_connection()?;
        let user_list = users
            .load::<crate::models::auth::User>(conn)
            .map_err(|e| FieldError::from(e))?;

        Ok(user_list.into_iter().map(UserType::from).collect())
    }
}
