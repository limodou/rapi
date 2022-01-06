use super::jwt_token::JwtUser;
use super::service;
use super::vo;
use crate::core::result;
use anyhow::anyhow;
use poem::{
  handler,
  web::{Data, Json},
  IntoResponse, Result,
};
use sqlx::{self, mysql::MySqlPool};
use validator::{Validate, ValidationError};

#[handler]
pub async fn login(
  Json(user): Json<vo::LoginReq>,
  Data(pool): Data<&MySqlPool>,
) -> Result<impl IntoResponse> {
  user
    .validate()
    .map_err(|e| anyhow!(format!("1006 {}", e.to_string())))?;
  let res = service::login(&pool, &user).await?;
  Ok(result::ok_data(res))
}

#[handler]
pub async fn register(
  Json(user): Json<vo::RegisterReq>,
  Data(pool): Data<&MySqlPool>,
) -> Result<impl IntoResponse> {
  service::register(&pool, user).await?;
  Ok(result::ok())
}

#[handler]
pub async fn get_user_info(
  JwtUser(user): JwtUser,
  Data(pool): Data<&MySqlPool>,
) -> Result<impl IntoResponse> {
  let res = service::get_user_info(&pool, &user).await?;
  Ok(result::ok_data(res))
}
