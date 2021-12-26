use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginUser {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
  pub username: String,
  pub realname: String,
  pub password: String,
}

