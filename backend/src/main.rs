#[macro_use]
extern crate log;

use std::sync::Arc;

use actix_files::Files;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Responder};
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::prelude::*;
use diesel::r2d2;
use dotenvy::dotenv;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use grooveguessr_backend::youtube::YoutubeClient;
use grooveguessr_backend::{
    auth, auth::create_client, auth::OpenIDConnectConfig, auth::UserInfo,
    auth_middleware::AuthRequired, youtube::Youtube, OidcClient,
};
use grooveguessr_backend::{AppState, DbPool, Mutation, Query};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

async fn graphql(
    context: Data<AppState>,
    session: Session,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();

    let user = session
        .get::<UserInfo>("user_info")
        .expect("Could not fetch user info - not logged in?")
        .unwrap();

    request = request.data(user);

    context.schema.execute(request).await.into()
}

async fn index_graphiql() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish())
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

fn initialize_redis() -> redis::Client {
    redis::Client::open(std::env::var("REDIS_URL").expect("REDIS_URL needs to be set"))
        .expect("Error initializing redis client")
}

async fn initialize_oidc_client() -> OidcClient {
    Arc::new(
        create_client(OpenIDConnectConfig {
            issuer_url: std::env::var("OIDC_ISSUER_URL").expect("OIDC_ISSUER_URL needs to be set"),
            client_id: std::env::var("OIDC_CLIENT_ID").expect("OIDC_CLIENT_ID needs to be set"),
            client_secret: std::env::var("OIDC_CLIENT_SECRET")
                .expect("OIDC_CLIENT_SECRET needs to be set"),
            redirect_url: std::env::var("OIDC_REDIRECT_URL")
                .expect("OIDC_REDIRECT_URL needs to be set"),
        })
        .await
        .expect("Error initializing OIDC client"),
    )
}

fn initialize_youtube() -> Youtube {
    Arc::new(YoutubeClient::new(
        std::env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY needs to be set"),
    ))
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

    let redis = initialize_redis();

    let secret_key = Key::from(
        std::env::var("SECRET_KEY")
            .expect("OIDC_CLIENT_SECRET needs to be set")
            .as_bytes(),
    );

    let oidc_client = initialize_oidc_client().await;
    let youtube_client = initialize_youtube();

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db_pool.clone())
        .data(youtube_client.clone())
        .data(redis.clone())
        .finish();

    let app_state = AppState {
        db_pool,
        redis,
        schema,
        oidc_client,
        youtube_client,
    };
    let app_data = Data::new(app_state);

    info!("GraphiQL IDE: http://localhost:8080/graphql");

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(1)))
                    .build(),
            )
            .app_data(app_data.clone())
            .service(web::resource("/login").to(auth::login))
            .service(web::resource("/auth_callback").to(auth::auth_callback))
            .service(web::resource("/logout").to(auth::logout))
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .wrap(AuthRequired)
                    .to(index_graphiql),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .wrap(AuthRequired)
                    .to(graphql),
            )
            .service(Files::new("/", "public").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
