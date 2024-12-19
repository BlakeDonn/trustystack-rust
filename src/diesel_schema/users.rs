// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        #[sql_name = "userId"]
        user_id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        provider -> Varchar,
        #[sql_name = "providerAccountId"]
        provider_account_id -> Varchar,
        refresh_token -> Nullable<Text>,
        access_token -> Nullable<Text>,
        expires_at -> Nullable<Int8>,
        id_token -> Nullable<Text>,
        scope -> Nullable<Text>,
        session_state -> Nullable<Text>,
        token_type -> Nullable<Text>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        #[sql_name = "userId"]
        user_id -> Int4,
        expires -> Timestamptz,
        #[sql_name = "sessionToken"]
        session_token -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        #[sql_name = "emailVerified"]
        email_verified -> Nullable<Timestamptz>,
        image -> Nullable<Text>,
    }
}

diesel::table! {
    verification_token (identifier, token) {
        identifier -> Text,
        expires -> Timestamptz,
        token -> Text,
    }
}

diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(accounts, sessions, users, verification_token,);
