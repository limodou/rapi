use poem::{web::{Data, Json}, IntoResponse, Result, handler};
use sqlx::{self, mysql::MySqlPool};
use super::service;
use super::vo;
use crate::core::result;


#[handler]
pub async fn login(
  Json(user): Json<vo::LoginReq>,
  Data(pool): Data<&MySqlPool>,
) -> Result<impl IntoResponse> {
  let res = service::login(&pool, user).await?;
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
