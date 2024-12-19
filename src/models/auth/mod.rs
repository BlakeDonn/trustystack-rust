use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::diesel_schema::users::users)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<DateTime<Utc>>,
    pub image: Option<String>,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = crate::diesel_schema::users::sessions)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub expires: DateTime<Utc>,
    pub session_token: String,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = crate::diesel_schema::users::accounts)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub type_: String,
    pub provider: String,
    pub provider_account_id: String,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<i64>,
    pub id_token: Option<String>,
    pub scope: Option<String>,
    pub session_state: Option<String>,
    pub token_type: Option<String>,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = crate::diesel_schema::users::verification_tokens)]
pub struct VerificationToken {
    pub identifier: String,
    pub token: String,
    pub expires: DateTime<Utc>,
}
