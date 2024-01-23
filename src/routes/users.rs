use axum::{extract::State, response::IntoResponse, Form};
use sqlx::query;

use crate::{
    error::{Error, Result},
    forms, utils, AppState,
};

pub(super) async fn get_user_info(
    State(state): State<AppState>,
    Form(payload): Form<forms::users::GetGJUserInfo>,
) -> Result<impl IntoResponse> {
    let own_profile = match (payload.account_id, payload.gjp2) {
        (Some(account_id), Some(gjp2)) => {
            let password = query!(
                "SELECT `password` FROM `users` WHERE `id` = ?",
                payload.account_id
            )
            .fetch_one(&state.pool)
            .await?
            .password;

            if password != gjp2 {
                return Err(Error::IncorrectGJP2);
            }

            account_id == payload.target_account_id
        }
        _ => false,
    };

    let user = query!(
        "SELECT * FROM `users` WHERE `id` = ?",
        payload.target_account_id
    )
    .fetch_one(&state.pool)
    .await?;

    let rank = query!(
        "SELECT COUNT(1) as rank FROM `users` WHERE `stars` > ?",
        user.stars
    )
    .fetch_one(&state.pool)
    .await?
    .rank
        + 1;

    let total_demons = user.easy_normal_demons
        + user.medium_normal_demons
        + user.hard_normal_demons
        + user.insane_normal_demons
        + user.extreme_normal_demons
        + user.easy_platformer_demons
        + user.medium_platformer_demons
        + user.hard_platformer_demons
        + user.insane_platformer_demons
        + user.extreme_platformer_demons
        + user.weekly_demons
        + user.gauntlet_demons;

    let demons_string = format!(
        "{},{},{},{},{},{},{},{},{},{},{},{}",
        user.easy_normal_demons,
        user.medium_normal_demons,
        user.hard_normal_demons,
        user.insane_normal_demons,
        user.extreme_normal_demons,
        user.easy_platformer_demons,
        user.medium_platformer_demons,
        user.hard_platformer_demons,
        user.insane_platformer_demons,
        user.extreme_platformer_demons,
        user.weekly_demons,
        user.gauntlet_demons
    );

    // TODO: add checks for message count, friend request count and new friends count
    let counts = if own_profile {
        format!("38:{}:39:{}:40:{}", 0, 0, 0)
    } else {
        "".into()
    };

    // TODO: add friendship check (31)
    Ok(format!("1:{}:2:{}:3:{}:4:{}:8:{}:10:{}:11:{}:13:{}:16:{}:17:{}:18:{}:19:{}:20:{}:21:{}:22:{}:23:{}:24:{}:25:{}:26:{}:28:{}:29:{}:30:{}:31:{}:{}:43:{}:44:{}:45:{}:46:{}:48:{}:49:{}:50:{}:51:{}:52:{}:53:{}:54:{}:55:{}",
                user.user_name, user.id, user.stars, total_demons, user.creator_points,
                user.primary_color, user.secondary_color, user.secret_coins, user.id, user.user_coins,
                user.message_state, user.friend_state, user.youtube_url, user.cube_id, user.ship_id,
                user.ball_id, user.ufo_id, user.wave_id, user.robot_id, user.glow_id,
                1, rank, 0, counts, user.spider_id, user.twitter_url,
                user.twitch_url, user.diamonds, user.explosion_id, user.mod_level, user.comment_history_state,
                user.tertiary_color, user.moons, user.swing_id, user.jetpack_id, demons_string))
}

pub(super) async fn update_user_score(
    State(state): State<AppState>,
    Form(payload): Form<forms::users::UpdateGJUserScore>,
) -> Result<impl IntoResponse> {
    // This is awful but apparently GD can send UpdateGJUserScore requests when you're not logged in.
    let (account_id, gjp2) = if payload.account_id.is_none() || payload.gjp2.is_none() {
        return Err(Error::IncorrectGJP2);
    } else {
        (payload.account_id.unwrap(), payload.gjp2.unwrap())
    };

    let result = query!(
        "SELECT `password` FROM `users` WHERE `id` = ?",
        payload.account_id
    )
    .fetch_one(&state.pool)
    .await?;

    if result.password != gjp2 {
        return Err(Error::IncorrectGJP2);
    }

    let chk = utils::generate_chk(
        &[
            &account_id,
            &payload.user_coins,
            &payload.demons,
            &payload.stars,
            &payload.secret_coins,
            &payload.icon_type,
            &payload.icon_id,
            &payload.diamonds,
            &payload.cube_id,
            &payload.ship_id,
            &payload.ball_id,
            &payload.ufo_id,
            &payload.wave_id,
            &payload.robot_id,
            &payload.glowing,
            &payload.spider_id,
            &payload.explosion_id,
        ],
        "xI35fsAapCRg",
        "85271",
    );

    if chk != payload.seed2 {
        return Err(Error::IncorrectChk);
    }

    query!(
        r"UPDATE `users` SET `stars` = ?, `moons` = ?, `diamonds` = ?, `icon_id` = ?, `primary_color` = ?, `secondary_color` = ?, `tertiary_color` = ?, `icon_type` = ?, `secret_coins` = ?, `user_coins` = ?, `glowing` = ?, `cube_id` = ?, `ship_id` = ?, `ball_id` = ?, `ufo_id` = ?, `wave_id` = ?, `robot_id` = ?, `glow_id` = ?, `spider_id` = ?, `explosion_id` = ?, `swing_id` = ?, `jetpack_id` = ? WHERE `id` = ?",
        payload.stars,
        payload.moons,
        payload.diamonds,
        payload.icon_id,
        payload.primary_color,
        payload.secondary_color,
        payload.tertiary_color,
        payload.icon_type,
        payload.secret_coins,
        payload.user_coins,
        payload.glowing,
        payload.cube_id,
        payload.ship_id,
        payload.ball_id,
        payload.ufo_id,
        payload.wave_id,
        payload.robot_id,
        payload.glow_id,
        payload.spider_id,
        payload.explosion_id,
        payload.swing_id,
        payload.jetpack_id,
        account_id
    )
    .execute(&state.pool)
    .await?;

    Ok(format!("{}", account_id))
}

pub(super) async fn update_account_settings(
    State(state): State<AppState>,
    Form(payload): Form<forms::users::UpdateGJAccSettings>,
) -> Result<impl IntoResponse> {
    let result = query!(
        "SELECT `password` FROM `users` WHERE `id` = ?",
        payload.account_id
    )
    .fetch_one(&state.pool)
    .await?;

    if result.password != payload.gjp2 {
        return Err(Error::IncorrectGJP2);
    }

    // ignoring errors because updateGJAccSettings returns 1 regardless of anything being updated
    _ = query!(
        "UPDATE `users` SET `message_state` = ?, `friend_state` = ?, `comment_history_state` = ?, `youtube_url` = ?, `twitter_url` = ?, `twitch_url` = ? WHERE `id` = ?", 
        payload.message_state, payload.friend_state, payload.comment_history_state, payload.youtube, payload.twitter, payload.twitch, payload.account_id
    )
        .execute(&state.pool)
        .await;

    Ok("1")
}
