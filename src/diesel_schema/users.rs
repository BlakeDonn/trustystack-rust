// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        user_id -> Int4,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
        #[max_length = 255]
        provider -> Varchar,
        #[max_length = 255]
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
        user_id -> Int4,
        expires -> Timestamptz,
        #[max_length = 255]
        session_token -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        email_verified -> Nullable<Timestamptz>,
        image -> Nullable<Text>,
    }
}

diesel::table! {
    verification_tokens (identifier, token) {
        identifier -> Text,
        token -> Text,
        expires -> Timestamptz,
    }
}

diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    sessions,
    users,
    verification_tokens,
);
