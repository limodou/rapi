use anyhow::{self, Result};
use super::model::*;
use super::vo::*;
use super::error::*;
use sqlx::{self, mysql::MySqlPool};
use crate::utils::jwt_middle::Token;
use chrono::Utc;

#[allow(dead_code)]
pub async fn login(pool: &MySqlPool, user: LoginReq) -> Result<LoginRes> {
  // 获取用户信息
  let u = User::find_by_username(pool, &user.username).await?;
  // 检查口令是否正确
  if u.password != user.password {
    return Err(InvalidPasswordError.into())
  }
  let claims = Token {
    sub: u.username,
    exp: Utc::now().timestamp() + 10000,
  };
  let token = claims.sign("hello")?;
  let res = LoginRes {
    token
  };
  Ok(res)
}

#[allow(dead_code)]
pub async fn register(pool: &MySqlPool, user: RegisterReq) -> Result<()> {
  User::create(&pool, &user).await?;
  Ok(())
}
