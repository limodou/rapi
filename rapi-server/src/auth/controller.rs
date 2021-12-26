use anyhow::Result;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use sqlx::{self, mysql::MySqlPool};

use super::User;
use crate::core::{result, Res, AppError};
use super::{LoginUser, RegisterUser};

pub async fn login(
  Json(user): Json<LoginUser>,
  Extension(pool): Extension<MySqlPool>,
) -> Result<impl IntoResponse, AppError> {
  let user = User::find_by_username(&pool, &user.username).await?;
  Ok(result::ok())
  // match User::find_by_username(&pool, &user.username).await {
  //   Ok(user) => {
  //     println!("{:?}", user);

  //     let res = Res::<String> {
  //       code: 0,
  //       data: Some("".into()),
  //       message: None,
  //     };

  //     Ok((StatusCode::OK, Json(res)))
  //   }
  //   Err(e) => {
  //     let res = Res::<String> {
  //       code: 1,
  //       data: None,
  //       message: Some(e.to_string()),
  //     };
  //     (StatusCode::OK, Json(res))
  //   }
  // }
}

pub async fn register(
  Json(user): Json<RegisterUser>,
  Extension(pool): Extension<MySqlPool>,
) -> impl IntoResponse {
  println!("user {:?}", user);

  match User::create(&pool, &user).await {
    Ok(_) => {
      println!("{:?}", user);

      let res = Res::<String> {
        code: 0,
        data: Some("".into()),
        message: None,
      };

      (StatusCode::CREATED, Json(res))
    }
    Err(e) => {
      let res = Res::<String> {
        code: 1,
        data: None,
        message: Some(e.to_string()),
      };
      (StatusCode::OK, Json(res))
    }
  }
}
