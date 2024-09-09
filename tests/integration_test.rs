use actix_web::{test, web, App};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use env_logger;
use rust_backend::graphql_handler;
use rust_backend::graphql_schema::context::Context; // Ensure you're importing Context from the correct module
use rust_backend::graphql_schema::create_schema;
use std::env;
use std::sync::Arc;

#[actix_rt::test]
async fn test_graphql_api_version() {
    std::env::set_var("RUST_LOG", "debug");
    dotenv().ok();
    env_logger::builder().is_test(true).init(); // Initialize env_logger with is_test set to true

    // Create schema
    let schema = Arc::new(create_schema());
    log::info!("Schema created successfully.");

    // Set up the database connection pool and context
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let context = web::Data::new(Context::new(pool)); // Wrap the context with `Data::new()`
    log::info!("Context created successfully.");

    // Set up the app with both the schema and context
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(schema.clone())) // Pass schema with Data::new()
            .app_data(context.clone()) // Pass context with Data::new()
            .service(
                actix_web::web::resource("/graphql")
                    .guard(actix_web::guard::Post())
                    .to(graphql_handler),
            ),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/graphql")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"query": "{ apiVersion }"}"#)
        .to_request();

    let response = test::call_service(&app, request).await;

    let status = response.status();
    let body = test::read_body(response).await;

    log::info!("Response status: {:?}", status);
    log::info!("Response body: {:?}", body);

    assert!(
        status.is_success(),
        "Expected success but got failure: {:?}",
        body
    );
    let expected_response = r#"{"data":{"apiVersion":"1.0"}}"#;
    assert_eq!(body, expected_response, "Unexpected response: {:?}", body);
}
