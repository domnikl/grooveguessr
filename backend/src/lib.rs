use diesel::PgConnection;

mod db_schema;
mod handler;
mod schemas;
mod services;

pub use crate::handler::graphql_handler::{Mutation, ProjectSchema, Query};

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;

pub struct AppContext {
    pub db_pool: DbPool,
    pub schema: ProjectSchema,
}

impl Clone for AppContext {
    fn clone(&self) -> Self {
        Self {
            db_pool: self.db_pool.clone(),
            schema: self.schema.clone(),
        }
    }
}
