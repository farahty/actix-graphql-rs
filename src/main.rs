mod config;
#[macro_use]
pub mod db;
mod handlers;
pub mod models;
mod schema;

use axum::{
    Extension, Router,
    routing::{get, post},
};
use config::AppConfig;
use mongodb::Client as MongoClient;
use mongodb::options::ClientOptions;
use redis::Client as RedisClient;
use redis::aio::ConnectionManager;
use std::sync::Arc;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = AppConfig::from_env();

    // Connect to Redis
    let redis_client =
        RedisClient::open(config.redis_url.as_str()).expect("Failed to create Redis client");
    let manager = ConnectionManager::new(redis_client)
        .await
        .expect("Failed to create Redis connection manager");
    let redis = Arc::new(manager);

    // Connect to MongoDB
    let mut client_options = ClientOptions::parse(&config.mongo_uri)
        .await
        .expect("Failed to parse MongoDB URI");
    client_options.app_name = Some("AxumPlayground".to_string());
    let mongo_client =
        MongoClient::with_options(client_options).expect("Failed to initialize MongoDB client");
    let db = mongo_client.database(&config.mongo_db);

    let schema = schema::build_schema(redis, &db);

    let app = Router::new()
        .route("/graphql", post(handlers::endpoint))
        .route("/", get(handlers::ui))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
