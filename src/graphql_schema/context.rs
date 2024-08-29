use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

/// Represents the context that holds the database connection pool.
pub struct Context {
    pub db: Pool<ConnectionManager<PgConnection>>,
}

impl Context {
    /// Creates a new context with the provided database connection pool.
    pub fn new(db: Pool<ConnectionManager<PgConnection>>) -> Self {
        log::info!("Creating new Context with database pool.");
        Context { db }
    }

    /// Retrieves a connection from the pool.
    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::r2d2::PoolError> {
        log::info!("Getting a database connection from the pool.");
        self.db.get()
    }
}

/// Required to implement Juniper's `Context` trait for integration with GraphQL.
impl juniper::Context for Context {}

