use anyhow::{self, Result};
use super::model::*;
use super::vo::*;
use sqlx::{self, mysql::MySqlPool};
use crate::core::AppError;
use super::error::AuthError;

#[allow(dead_code)]
pub async fn login(pool: &MySqlPool, user: LoginReq) -> Result<LoginRes, AppError> {
  // 获取用户信息
  let u = User::find_by_username(pool, &user.username).await?;
  // 检查口令是否正确
  if u.password != user.password {
    return Err(AuthError::invalid_password())
  }
  let claims = Token {
    sub: u.username,
    exp: 10000,
  };
  let token = claims.sign("hello")?;
  let res = LoginRes {
    token
  };
  Ok(res)
}

#[allow(dead_code)]
pub async fn register(pool: &MySqlPool, user: RegisterReq) -> Result<(), AppError> {
  User::create(&pool, &user).await?;
  Ok(())
}
