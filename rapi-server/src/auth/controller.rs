use anyhow::Result;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use sqlx::{self, mysql::MySqlPool};

use super::User;
use super::{LoginUser, RegisterUser};
use crate::core::{result, AppError, Res};

pub async fn login(
  Json(user): Json<LoginUser>,
  Extension(pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, AppError> {
  User::find_by_username(&pool, &user.username).await?;
  Ok(result::ok())
}

pub async fn register(
  Json(user): Json<RegisterUser>,
  Extension(pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, AppError> {
  println!("user {:?}", user);

  User::create(&pool, &user).await?;
  Ok(result::ok())
}
