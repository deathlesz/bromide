use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct RegisterGJAccount {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub secret: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct LoginGJAccount {
    pub udid: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub gjp2: String,
    pub secret: String,
}
