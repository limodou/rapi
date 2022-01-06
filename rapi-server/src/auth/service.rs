use super::error::*;
use super::model::*;
use super::token::Token;
use super::vo::*;
use anyhow::{anyhow, Result};
use sqlx::{self, mysql::MySqlPool};

#[allow(dead_code)]
pub async fn login(pool: &MySqlPool, user: &LoginReq) -> Result<LoginRes> {
  // 获取用户信息
  let u = User::find_by_username(pool, &user.username).await?;
  if u.is_none() {
    return Err(anyhow::anyhow!("1007 用户登录失败"));
  }
  let u = u.unwrap();
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

#[allow(dead_code)]
pub async fn get_user_info(pool: &MySqlPool, user: &User) -> Result<GetUserInfoRes> {
  // let user = User::find_by_username(&pool, &user.username).await?;
  let res = GetUserInfoRes {
    username: user.username.clone(),
    realname: user.realname.clone(),
    is_super: user.is_super,
  };
  Ok(res)
}
