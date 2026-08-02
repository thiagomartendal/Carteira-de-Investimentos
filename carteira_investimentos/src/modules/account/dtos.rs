use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
    pub save_session: Option<String>
}

#[derive(Deserialize)]
pub struct NewAccountForm {
    pub email: String,
    pub username: String,
    pub password: String,
    pub user_type: i32
}
