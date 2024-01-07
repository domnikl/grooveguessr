use std::sync::Arc;

use diesel::PgConnection;
use openidconnect::core::CoreClient;
use youtube::Youtube;

pub mod auth;
pub mod auth_middleware;
mod db_schema;
mod handler;
mod models;
mod services;
pub mod youtube;

pub use crate::handler::graphql_handler::{Mutation, ProjectSchema, Query};

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type OidcClient = Arc<CoreClient>;

pub struct AppState {
    pub db_pool: DbPool,
    pub schema: ProjectSchema,
    pub oidc_client: OidcClient,
    pub youtube_client: Youtube,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            db_pool: self.db_pool.clone(),
            schema: self.schema.clone(),
            oidc_client: self.oidc_client.clone(),
            youtube_client: self.youtube_client.clone(),
        }
    }
}
