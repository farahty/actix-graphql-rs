use crate::config::AppConfig;

pub mod repo;
#[macro_use]
pub mod model_macros;

pub async fn connect_db(config: &AppConfig) -> mongodb::Database {
    let mut client_options = mongodb::options::ClientOptions::parse(&config.mongo_uri)
        .await
        .expect("Failed to parse MongoDB URI");

    client_options.app_name = Some("AxumPlayground".to_string());
    let client =
        mongodb::Client::with_options(client_options).expect("Failed to initialize MongoDB client");

    client.database(&config.mongo_db)
}

pub async fn connect_redis(config: &AppConfig) -> redis::aio::ConnectionManager {
    let client =
        redis::Client::open(config.redis_url.as_str()).expect("Failed to create Redis client");
    redis::aio::ConnectionManager::new(client)
        .await
        .expect("Failed to create Redis connection manager")
}
