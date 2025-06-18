use dotenvy;
use std::env;

pub struct AppConfig {
    pub redis_url: String,
    pub mongo_uri: String,
    pub mongo_db: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            mongo_uri: env::var("MONGO_URI").expect("MONGO_URI must be set"),
            mongo_db: env::var("MONGO_DB").unwrap_or_else(|_| "actix_playground".to_string()),
        }
    }
}
