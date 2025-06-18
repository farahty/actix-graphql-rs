mod config;
pub mod db;
mod handlers;
mod schema;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use mongodb::options::ClientOptions;
use mongodb::Client as MongoClient;
use redis::aio::ConnectionManager;
use redis::Client as RedisClient;
use std::sync::Arc;

use config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    client_options.app_name = Some("ActixPlayground".to_string());

    let mongo_client =
        MongoClient::with_options(client_options).expect("Failed to initialize MongoDB client");

    let db = mongo_client.database(&config.mongo_db);

    let schema = schema::build_schema(redis, &db);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::new(schema.clone()))
            .service(handlers::endpoint)
            .service(handlers::ui)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
