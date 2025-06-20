mod config;
#[macro_use]
pub mod db;
mod handlers;
pub mod models;
mod schema;

pub mod error;

use async_graphql_axum::GraphQL;
use axum::{Router, routing::get};
use config::AppConfig;
use std::fs::File;
use std::io::Write;
use std::{error::Error, sync::Arc};
use tracing_subscriber;

use crate::db::{connect_db, connect_redis};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let config = AppConfig::from_env();
    let redis = Arc::new(connect_redis(&config).await);
    let db = connect_db(&config).await;
    let schema = schema::build_schema(redis, &db);
    File::create("app.schema.gql")?.write_all(&schema.sdl().as_bytes())?;

    let app = Router::new().route("/", get(handlers::ui).post_service(GraphQL::new(schema)));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
