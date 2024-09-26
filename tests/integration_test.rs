mod utils;

use actix_web::test;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

#[actix_rt::test]
async fn test_database_connection() {
    dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    println!("Database url: {:?}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let _conn = pool.get().expect("Failed to get a database connection");
    println!("Successfully connected to the database");
}

#[actix_rt::test]
async fn test_graphql_api_version() {
    let app = setup_test_app!();

    let request = test::TestRequest::post()
        .uri("/graphql")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"query": "{ apiVersion }"}"#)
        .to_request();

    let response = test::call_service(&app, request).await;
    let body = test::read_body(response).await;

    assert_eq!(body, r#"{"data":{"apiVersion":"1.0"}}"#);
}

#[actix_rt::test]
async fn test_root_query_api_version() {
    let app = setup_test_app!();

    let request = test::TestRequest::post()
        .uri("/graphql")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"query": "{ apiVersion }"}"#)
        .to_request();

    let response = test::call_service(&app, request).await;
    let body = test::read_body(response).await;

    assert_eq!(body, r#"{"data":{"apiVersion":"1.0"}}"#);
}
