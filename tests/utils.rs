// tests/utils.rs

#[macro_export]
macro_rules! setup_test_app {
    () => {{
        use actix_web::{test, web, App};
        use diesel::r2d2::{ConnectionManager, Pool};
        use diesel::PgConnection;
        use dotenv::dotenv;
        use rust_backend::graphql_handler::graphql_handler;
        use rust_backend::graphql_schema::{context::Context, schema::create_schema};
        use std::env;
        use std::sync::Arc;

        dotenv().ok();

        let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        let context = web::Data::new(Context::new(pool, None));

        // Create GraphQL schema
        let schema = Arc::new(create_schema());

        // Initialize the Actix Web application
        test::init_service(
            App::new()
                .app_data(web::Data::new(schema.clone()))
                .app_data(context.clone())
                .service(
                    web::resource("/graphql")
                        .guard(actix_web::guard::Post())
                        .to(graphql_handler),
                ),
        )
        .await
    }};
}
