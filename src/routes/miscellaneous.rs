use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::IntoResponse,
    Form,
};
use sqlx::query;
use tracing::debug;

use crate::{
    error::{Error, Result},
    forms, utils, AppState,
};

pub(super) async fn like_item(
    State(state): State<AppState>,
    Form(payload): Form<forms::miscellaneous::LikeGJItem>,
) -> Result<impl IntoResponse> {
    match (payload.account_id, payload.gjp2) {
        (Some(account_id), Some(gjp2)) => {
            let result = query!("SELECT `password` FROM `users` WHERE `id` = ?", account_id)
                .fetch_one(&state.pool)
                .await?;

            if result.password != gjp2 {
                return Err(Error::IncorrectGJP2);
            }
        }
        (None, None) => {}
        _ => return Err(Error::IncorrectGJP2),
    };

    let chk = utils::generate_chk(
        &[
            &payload.special,
            &payload.item_id,
            &payload.is_like,
            &payload.r#type,
            &payload.rs,
            &payload.account_id.unwrap_or(0),
            &payload.udid,
            &payload.uuid,
        ],
        "ysg6pUrtjn0J",
        "58281",
    );

    if chk != payload.chk {
        return Err(Error::IncorrectChk);
    }

    if payload.r#type != 3 {
        unimplemented!()
    }

    if payload.is_like == 0 {
        _ = query!(
            "UPDATE `account_comments` SET `dislikes` = `dislikes` + 1 WHERE `id` = ?",
            payload.item_id
        )
        .execute(&state.pool)
        .await;
    } else {
        _ = query!(
            "UPDATE `account_comments` SET `likes` = `likes` + 1 WHERE `id` = ?",
            payload.item_id
        )
        .execute(&state.pool)
        .await;
    }

    Ok("1")
}
pub(super) async fn unhandled(uri: Uri, body: String) -> StatusCode {
    debug!("Got unhandled request:");
    debug!("\tURI: {:?}", uri);

    debug!("\tParams:");
    body.split('&').for_each(|pair| debug!("\t\t{pair}"));

    StatusCode::IM_A_TEAPOT
}
