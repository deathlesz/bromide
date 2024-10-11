use axum::{extract::State, response::IntoResponse, routing::post, Router};

use crate::{error::Result, form::Form, schema, utils};

async fn update_user(
    State(pool): State<sqlx::PgPool>,
    Form(data): Form<schema::UpdateGJUserScore22>,
) -> Result<impl IntoResponse> {
    if data.secret != "Wmfd2893gb7" {
        return Err("-1")?;
    }

    let password = sqlx::query_scalar!(
        "SELECT password FROM accounts WHERE id = $1 AND user_name = $2",
        data.account_id,
        data.user_name,
    )
    .fetch_one(&pool)
    .await?;

    if !utils::check_password(&password, &data.gjp2) {
        return Err("-1")?;
    }

    // TODO: maybe there is a better way?
    let chk = utils::generate_chk(
        &[
            &data.account_id,
            &data.user_coins,
            &data.demons,
            &data.stars,
            &data.secret_coins,
            &data.icon_type,
            &data.icon_id,
            &data.diamonds,
            &data.cube,
            &data.ship,
            &data.ball,
            &data.ufo,
            &data.wave,
            &data.robot,
            &data.is_glowing, // maybe wrong?
            &data.spider,
            &data.explosion,
            &data.demon_info.unwrap_or_default(),
            &data.weekly_info.map(|v| v.to_string()).unwrap_or_default(),
            &data
                .gauntlet_demon_info
                .map(|v| v.to_string())
                .unwrap_or_default(),
            &data.level_info,
            &data.daily_info,
            &data.gauntlet_info,
        ],
        "xI35fsAapCRg",
        "85271",
    );

    if chk != data.seed2 {
        return Err("-1")?;
    }

    let id = sqlx::query_scalar!("UPDATE users SET stars = $1, moons = $2, diamonds = $3, icon_id = $4, icon_type = $5, secret_coins = $6, user_coins = $7, cube_id = $8, ship_id = $9, ball_id = $10, ufo_id = $11, wave_id = $12, robot_id = $13, glow_id = $14, spider_id = $15, explosion_id = $16, swing_id = $17, jetpack_id = $18 WHERE account_id = $19 RETURNING id", 
        data.stars,
        data.moons,
        data.diamonds,
        data.icon_id,
        data.icon_type,
        data.secret_coins,
        data.user_coins,
        data.cube,
        data.ship,
        data.ball,
        data.ufo,
        data.wave,
        data.robot,
        data.glow,
        data.spider,
        data.explosion,
        data.swing,
        data.jetpack,
        data.account_id,
    ).fetch_one(&pool).await?;

    Ok(id.to_string())
}

async fn get_user_info(
    State(pool): State<sqlx::PgPool>,
    Form(data): Form<schema::GetGJUserInfo20>,
) -> Result<impl IntoResponse> {
    if data.secret != "Wmfd2893gb7" {
        return Err("-1")?;
    }

    let target = data.target_account_id;
    let is_own_profile = match (data.account_id, data.gjp2) {
        (Some(account_id), Some(gjp2)) => {
            let password =
                sqlx::query_scalar!("SELECT password FROM accounts WHERE id = $1", account_id)
                    .fetch_one(&pool)
                    .await?;

            if !utils::check_password(&password, &gjp2) {
                return Err("-1")?;
            }

            account_id == target
        }
        _ => false,
    };

    let user = sqlx::query!(
        "SELECT u.*, a.user_name FROM users AS u JOIN accounts AS a ON u.account_id = a.id WHERE a.id = $1",
        target,
    )
    .fetch_one(&pool)
    .await?;

    // TODO: add checks for message count, friend request count and new friends count
    let counts = if is_own_profile {
        format!("38:{}:39:{}:40:{}", 0, 0, 0)
    } else {
        "".into()
    };

    Ok(
        format!("1:{}:2:{}:3:{}:4:{}:8:{}:9:{}:10:{}:11:{}:13:{}:14:{}:15:{}:16:{}:17:{}:18:{}:19:{}:20:{}:21:{}:22:{}:23:{}:24:{}:25:{}:26:{}:28:{}:29:1:30:{}:31:{}:{}:40:{}:41:{}:42:{}43:{}:44:{}:45:{}:46:{}:48:{}:49:{}:50:{}:51:{}:52:{}:53:{}:54:{}:55:{}:56:{}:57:{}", user.user_name, user.id, user.stars, 0 /* demons */, user.creator_points, user.icon_id,
                user.primary_color, user.secondary_color, user.secret_coins, user.icon_type, if user.glowing { 2 } else { 0}, user.account_id, user.user_coins,
                user.message_state, user.friend_state, user.youtube_url, user.cube_id, user.ship_id,
                user.ball_id, user.ufo_id, user.wave_id, user.robot_id, user.glow_id,
                1 /* rank */, 0, counts, 0, 0, "???", user.spider_id, user.twitter_url,
                user.twitch_url, user.diamonds, user.explosion_id, user.mod_level, user.comment_history_state,
                user.tertiary_color, user.moons, user.swing_id, user.jetpack_id, "???", "???", "???"),
    )
}

pub(super) fn router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/updateGJUserScore22.php", post(update_user))
        .route("/getGJUserInfo20.php", post(get_user_info))
        .with_state(pool)
}
