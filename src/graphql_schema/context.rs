use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub struct Context {
    pub db: Pool<ConnectionManager<PgConnection>>,
}

impl Context {
    pub fn new(db: Pool<ConnectionManager<PgConnection>>) -> Self {
        Context { db }
    }

    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::r2d2::PoolError> {
        self.db.get()
    }
}

impl juniper::Context for Context {}
