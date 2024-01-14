use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct RegisterGJAccount {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LoginGJAccount {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub gjp2: String,
}
