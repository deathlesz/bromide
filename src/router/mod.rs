use axum::Router;

mod accounts;
mod users;

pub fn router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .merge(accounts::router(pool.clone()))
        .merge(users::router(pool.clone()))
}
