use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct LoginReq {
  #[validate(length(min=5, message="用户名长度需要大于等于5"))]
  pub username: String,
  #[validate(length(min=4))]
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRes {
  pub token: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterReq {
  #[validate(length(min=5))]
  pub username: String,
  #[validate(length(min=5))]
  pub realname: String,
  #[validate(length(min=4))]
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct GetUserInfoRes {
  pub username: String,
  pub realname: String,
  pub is_super: bool,
}
