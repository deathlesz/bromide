use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct RegisterGJAccount {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub secret: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct LoginGJAccount {
    pub udid: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    pub gjp2: String,
    pub secret: String,
}
