use anyhow::Result;
use axum::{extract::Extension, response::IntoResponse, Json};
use sqlx::{self, mysql::MySqlPool};

use super::vo::{LoginReq, RegisterReq};
use crate::core::{result, AppError};
use super::service;

pub async fn login(
  Json(user): Json<LoginReq>,
  Extension(pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, AppError> {
  let res = service::login(&pool, user).await?;
  Ok(result::ok_data(res))
}

pub async fn register(
  Json(user): Json<RegisterReq>,
  Extension(pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, AppError> {
  service::register(&pool, user).await?;
  Ok(result::ok())
}
