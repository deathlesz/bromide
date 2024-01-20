use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub(crate) struct GetGJAccountComments {
    pub page: u16,
    // GD sents accountID twice (presumably one is sender account ID and the other is target account ID)
    // This workaround is needed because serde can't parse duplicate fields
    // TODO: find a better way to do this
    #[serde(flatten)]
    extras: HashMap<String, String>,
}

impl GetGJAccountComments {
    pub fn account_id(&self) -> Option<u32> {
        self.extras
            .get("accountID")?
            .parse()
            .map(Some)
            .unwrap_or(None)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct UploadGJAccComment {
    #[serde(rename = "accountID")]
    pub account_id: u32,
    pub gjp2: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub comment: String, // encoded with urlsafe base64
    pub chk: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DeleteGJAccComment {
    pub gjp2: String,
    #[serde(rename = "targetAccountID")]
    pub target_account_id: u32,
    #[serde(rename = "commentID")]
    pub comment_id: u32,
}
