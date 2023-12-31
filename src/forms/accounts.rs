use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterGJAccount {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub email: String,
    pub password: String,
}
