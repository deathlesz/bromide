use axum::{routing::any, Router};

mod miscellaneous;

pub fn router() -> Router {
    Router::new()
        .route("/*rest", any(miscellaneous::unhandled))
}
