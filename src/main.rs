mod config;
#[macro_use]
pub mod db;
mod error;
mod handlers;
mod models;
mod schema;
pub mod utils;

use crate::db::{connect_db, connect_redis};
use async_graphql_axum::GraphQL;
use axum::{Router, routing::get};
use config::AppConfig;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Load application configuration from environment variables
    let config = AppConfig::from_env();

    // Connect to Redis and MongoDB
    let redis = connect_redis(&config).await;
    let db = connect_db(&config).await;

    // Build the GraphQL schema and write SDL to a file for tooling
    let schema = schema::build_schema(redis, db);
    File::create("app.schema.gql")?.write_all(&schema.sdl().as_bytes())?;

    // Set up the Axum router with GraphQL and UI endpoints
    let app = Router::new().route("/", get(handlers::ui).post_service(GraphQL::new(schema)));

    // Start the HTTP server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
