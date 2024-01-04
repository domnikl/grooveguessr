#[macro_use]
extern crate log;

use actix_files::Files;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::prelude::*;
use diesel::r2d2;
use dotenvy::dotenv;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::handler::graphql_handler::{ProjectSchema, Query};

mod handler;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct AppContext {
    db_pool: DbPool,
    schema: ProjectSchema,
}

impl Clone for AppContext {
    fn clone(&self) -> Self {
        Self {
            db_pool: self.db_pool.clone(),
            schema: self.schema.clone(),
        }
    }
}

async fn graphql(
    context: Data<AppContext>,
    _session: Session,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let request = req.into_inner();

    context.schema.execute(request).await.into()
}

async fn index_graphiql() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish())
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
fn initialize_db_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Error building r2d2 pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // initialize outside of `HttpServer::new` so that it is shared across all workers
    let db_pool = initialize_db_pool();
    db_pool
        .get()
        .unwrap()
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running pending migrations");

    let secret_key = Key::from(
        std::env::var("SECRET_KEY")
            .expect("OIDC_CLIENT_SECRET needs to be set")
            .as_bytes(),
    );

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db_pool.clone())
        .finish();

    let app_context = AppContext { db_pool, schema };

    info!("GraphiQL IDE: http://localhost:8080/graphql");

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(1)))
                    .build(),
            )
            .app_data(Data::new(app_context.clone()))
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(index_graphiql),
            )
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            .service(Files::new("/", "public").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
