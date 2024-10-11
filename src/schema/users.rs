use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UpdateGJUserScore22 {
    #[serde(rename = "accountID")]
    pub account_id: i64,
    pub gjp2: String,
    #[serde(rename = "userName")]
    pub user_name: Option<String>,

    pub stars: i64,
    pub moons: i64,
    pub demons: i64,
    pub diamonds: i64,

    #[serde(rename = "icon")]
    pub icon_id: i16,
    #[serde(rename = "color1")]
    pub primary_color: u8,
    #[serde(rename = "color2")]
    pub secondary_color: u8,
    #[serde(rename = "color3")]
    pub tertiary_color: i16, // -1?
    #[serde(rename = "iconType")]
    pub icon_type: i16,

    #[serde(rename = "coins")]
    pub secret_coins: i64,
    #[serde(rename = "userCoins")]
    pub user_coins: i64,

    #[serde(rename = "special")]
    pub is_glowing: i64,

    pub secret: String,

    #[serde(rename = "accIcon")]
    pub cube: i16,
    #[serde(rename = "accShip")]
    pub ship: i16,
    #[serde(rename = "accBall")]
    pub ball: i16,
    #[serde(rename = "accBird")]
    pub ufo: i16,
    #[serde(rename = "accDart")]
    pub wave: i16,
    #[serde(rename = "accRobot")]
    pub robot: i16,
    #[serde(rename = "accGlow")]
    pub glow: i16,
    #[serde(rename = "accSpider")]
    pub spider: i16,
    #[serde(rename = "accExplosion")]
    pub explosion: i16,
    #[serde(rename = "accSwing")]
    pub swing: i16,
    #[serde(rename = "accJetpack")]
    pub jetpack: i16,

    #[serde(rename = "dinfo")]
    pub demon_info: Option<String>,
    #[serde(rename = "dinfow")]
    pub weekly_info: Option<i64>,
    #[serde(rename = "dinfog")]
    pub gauntlet_demon_info: Option<i64>,
    #[serde(rename = "sinfo")]
    pub level_info: String,
    #[serde(rename = "sinfod")]
    pub daily_info: i64,
    #[serde(rename = "sinfog")]
    pub gauntlet_info: i64,

    pub seed: String,
    pub seed2: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetGJUserInfo20 {
    #[serde(rename = "accountID")]
    pub account_id: Option<i64>,
    #[serde(rename = "targetAccountID")]
    pub target_account_id: i64,
    pub gjp2: Option<String>,
    pub secret: String,
}
