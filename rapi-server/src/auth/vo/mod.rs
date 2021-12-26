use serde::{Serialize, Deserialize};
mod token;
pub use token::Token;

#[derive(Debug, Deserialize)]
pub struct LoginReq {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRes {
  pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterReq {
  pub username: String,
  pub realname: String,
  pub password: String,
}
