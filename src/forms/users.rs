use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GetGJUserInfo {
    #[serde(rename = "accountID")]
    pub account_id: Option<u32>,
    #[serde(rename = "targetAccountID")]
    pub target_account_id: u32,
    pub gjp2: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateGJUserScore {
    #[serde(rename = "accountID")]
    pub account_id: Option<u32>,
    pub gjp2: Option<String>,
    pub stars: u32,
    pub moons: u32,
    pub demons: u32,
    pub diamonds: u32,
    #[serde(rename = "icon")]
    pub icon_id: u16,
    #[serde(rename = "color1")]
    pub primary_color: u16,
    #[serde(rename = "color2")]
    pub secondary_color: u16,
    #[serde(rename = "color3")]
    pub tertiary_color: i16,
    #[serde(rename = "iconType")]
    pub icon_type: u8,
    #[serde(rename = "coins")]
    pub secret_coins: u16,
    #[serde(rename = "userCoins")]
    pub user_coins: u16,
    #[serde(rename = "special")]
    pub glowing: u8,
    #[serde(rename = "accIcon")]
    pub cube_id: u16,
    #[serde(rename = "accShip")]
    pub ship_id: u16,
    #[serde(rename = "accBall")]
    pub ball_id: u16,
    #[serde(rename = "accBird")]
    pub ufo_id: u16,
    #[serde(rename = "accDart")]
    pub wave_id: u16,
    #[serde(rename = "accRobot")]
    pub robot_id: u16,
    #[serde(rename = "accGlow")]
    pub glow_id: u16,
    #[serde(rename = "accSpider")]
    pub spider_id: u16,
    #[serde(rename = "accExplosion")]
    pub explosion_id: u16,
    #[serde(rename = "accSwing")]
    pub swing_id: u16,
    #[serde(rename = "accJetpack")]
    pub jetpack_id: u16,
    pub seed2: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateGJAccSettings {
    #[serde(rename = "accountID")]
    pub account_id: u32,
    pub gjp2: String,
    #[serde(rename = "mS")]
    pub message_state: u8,
    #[serde(rename = "frS")]
    pub friend_state: u8,
    #[serde(rename = "cS")]
    pub comment_history_state: u8,
    #[serde(rename = "yt")]
    pub youtube: String,
    pub twitter: String,
    pub twitch: String,
}
