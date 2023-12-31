use sqlx::SqlitePool;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    config: Config,
    pool: SqlitePool,
}

impl AppState {
    pub fn new(config: Config, pool: SqlitePool) -> Self {
        Self { config, pool }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
