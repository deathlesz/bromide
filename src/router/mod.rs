use axum::Router;

mod accounts;

pub fn router(pool: sqlx::PgPool) -> Router {
    Router::new().merge(accounts::router(pool.clone()))
}
