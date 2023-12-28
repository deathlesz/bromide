use axum::{routing::any, Router};

use crate::config;

mod miscellaneous;

pub fn router(config: config::Config) -> Router {
    Router::new()
        .route("/*rest", any(miscellaneous::unhandled))
        .with_state(config)
}
