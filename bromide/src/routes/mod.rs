use axum::{
    routing::{any, post},
    Router,
};

use crate::state::AppState;

mod accounts;
mod miscellaneous;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest(
            state.config().api_url(),
            Router::new()
                .route("/accounts/registerGJAccount.php", post(accounts::register))
                .route("/accounts/loginGJAccount.php", post(accounts::login)),
        )
        .route("/*rest", any(miscellaneous::unhandled))
        .with_state(state)
}
