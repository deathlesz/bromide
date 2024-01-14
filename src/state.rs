use sqlx::SqlitePool;

use crate::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: SqlitePool,
}
