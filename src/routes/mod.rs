use axum::{Router, routing::any};

mod miscellaneous;

pub fn router() -> Router {
    Router::new()
        .route("/*rest", any(miscellaneous::unhandled))
}
