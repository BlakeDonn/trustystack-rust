use actix_web::{test, App};
use std::sync::Arc;
use rust_backend::{schema, graphql_handler}; // Adjust the path based on your project structure

#[actix_rt::test]
async fn test_graphql_endpoint() {
    let schema = Arc::new(schema::create_schema());

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(schema.clone()))
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
    assert!(response.status().is_success());

    let body = test::read_body(response).await;
    let expected_response = r#"{"data":{"apiVersion":"1.0"}}"#;
    assert_eq!(body, expected_response);
}

