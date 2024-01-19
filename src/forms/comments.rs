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
