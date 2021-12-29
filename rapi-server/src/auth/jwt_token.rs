use super::jwt_middle::UserId;
use super::model::User;
use anyhow::anyhow;
use poem::{error::Error, FromRequest, Request, RequestBody, Result};
use sqlx::{self, mysql::MySqlPool};

#[derive(Debug)]
pub struct JwtUser(pub User);

// Implements a token extractor
#[poem::async_trait]
impl<'a> FromRequest<'a> for JwtUser {
  async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
    let pool = req.extensions().get::<MySqlPool>();
    if pool.is_none() {
      return Err(Error::from(anyhow!(
        "1005 Database connection pool is missing"
      )));
    }
    let pool = pool.unwrap();
    let user = match req.extensions().get::<UserId>() {
      Some(user) => {
        println!("user={:?}", user);
        let u = (*user).clone();
        match u.0 {
          Some(username) => User::find_by_username(pool, &username).await?,
          None => {
            return Err(Error::from(anyhow!(
              "1004 Token is not valid or missing token"
            )))
          }
        }
      }
      None => {
        return Err(Error::from(anyhow!(
          "1004 Token is not valid or missing token"
        )));
      }
    };
    Ok(JwtUser(user))
  }
}

// 不校验 JWT Token， 如果不存在则返回 None
#[derive(Debug)]
pub struct JwtUserNotCheck(pub Option<User>);

// Implements a token extractor
#[poem::async_trait]
impl<'a> FromRequest<'a> for JwtUserNotCheck {
  async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
    let pool = req.extensions().get::<MySqlPool>();
    if pool.is_none() {
      return Err(Error::from(anyhow!(
        "1005 Database connection pool is missing"
      )));
    }
    let pool = pool.unwrap();
    let user = match req.extensions().get::<UserId>() {
      Some(user) => {
        println!("user={:?}", user);
        let u = (*user).clone();
        match u.0 {
          Some(user) => Some(User::find_by_username(pool, &user).await?),
          None => None,
        }
      }
      None => None,
    };
    Ok(JwtUserNotCheck(user))
  }
}
