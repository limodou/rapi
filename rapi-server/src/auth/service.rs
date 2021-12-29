use super::error::*;
use super::model::*;
use super::vo::*;
use crate::utils::token::Token;
use anyhow::{self, Result};
use sqlx::{self, mysql::MySqlPool};

#[allow(dead_code)]
pub async fn login(pool: &MySqlPool, user: LoginReq) -> Result<LoginRes> {
  // 获取用户信息
  let u = User::find_by_username(pool, &user.username).await?;
  // 检查口令是否正确
  if u.password != user.password {
    return Err(AuthError::new(AuthErrorKind::InvalidPasswordError));
  }
  let claims = Token::new(u.username, 10000);
  let token = claims.sign()?;
  let res = LoginRes { token };
  Ok(res)
}

#[allow(dead_code)]
pub async fn register(pool: &MySqlPool, user: RegisterReq) -> Result<()> {
  User::create(&pool, &user).await?;
  Ok(())
}
