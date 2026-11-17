use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            host: std::env::var("BELHANA_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: std::env::var("BELHANA_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            jwt_secret: std::env::var("BELHANA_JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-me".into()),
            database_url: std::env::var("BELHANA_DATABASE_URL").unwrap_or_else(|_| "belhana.db".into()),
        }
    }
}

pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub config: Config,
}

pub type SharedState = Arc<AppState>;
