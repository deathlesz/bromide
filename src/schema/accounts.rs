use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct RegisterGJAccount {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct LoginGJAccount {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub gjp2: String,
}
