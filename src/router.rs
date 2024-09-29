use axum::Router;

mod accounts;

pub fn router(pool: sqlx::AnyPool) -> Router {
    Router::new().merge(accounts::router(pool.clone()))
}
