use axum::{
    routing::{get, post},
    Router, Server,
};
use http::Method;
use std::str::FromStr;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors, trace::TraceLayer};

use core::sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

pub mod api;
pub mod cookies;
pub mod middleware;
use crate::api::{create_message, delete_message, get_message, list_messages, update_message};
use crate::cookies::handle_cookies;
use crate::middleware::print_request_response;

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    // let cors =

    let app = Router::new()
        .route("/", get(list_messages).post(create_message))
        .route("/:id", get(get_message).post(update_message))
        .route("/delete/:id", post(delete_message))
        .layer(
            ServiceBuilder::new()
                .layer(CookieManagerLayer::new())
                .layer(axum::middleware::from_fn(print_request_response))
                .layer(TraceLayer::new_for_http())
                .layer(
                    cors::CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST])
                        // allow requests from any origin
                        .allow_origin(cors::Any),
                )
                .layer(axum::middleware::from_fn(handle_cookies)),
        )
        .with_state(state);

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}
