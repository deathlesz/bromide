use axum::{extract::State, response::IntoResponse, routing::post, Form, Router};

use crate::schema;

async fn register(
    State(pool): State<sqlx::PgPool>,
    Form(data): Form<schema::RegisterGJAccount>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if !data.user_name.chars().all(char::is_alphanumeric) {
        return Err("-1");
    } else if data.user_name.len() < 3 {
        return Err("-9");
    } else if data.user_name.len() > 20 {
        return Err("-4");
    } else if !data
        .password
        .chars()
        .all(|char| char.is_alphanumeric() || char == '-' || char == '_')
    {
        return Err("-5");
    } else if data.password.len() < 3 {
        return Err("-8");
    } else if data.password.len() > 20 {
        return Err("-5");
    }
    // } else if !email_address::EmailAddress::is_valid(&payload.email) {
    //     return Err("-6");
    // }

    Ok(())
}

pub(super) fn router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/accounts/registerGJAccount.php", post(register))
        .with_state(pool)
}
