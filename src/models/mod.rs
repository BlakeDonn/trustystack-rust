pub mod parts;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub struct Context {
    pub db: Pool<ConnectionManager<PgConnection>>,
}

impl Context {
    pub fn new(db: Pool<ConnectionManager<PgConnection>>) -> Self {
        log::info!("Creating new Context with database pool.");
        Context { db }
    }

    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::r2d2::PoolError> {
        log::info!("Getting a database connection from the pool.");
        self.db.get()
    }
}

impl juniper::Context for Context {}
