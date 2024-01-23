use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct LikeGJItem {
    pub udid: String,
    pub uuid: u32,
    #[serde(rename = "accountID")]
    pub account_id: Option<u32>,
    pub gjp2: Option<String>,
    #[serde(rename = "itemID")]
    pub item_id: u32,
    #[serde(rename = "like")]
    pub is_like: u8,
    pub r#type: u8,
    pub special: u8,
    pub rs: String,
    pub chk: String,
}
