// use thiserror::Error;
use anyhow;
use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use super::result;

#[derive(Debug)]
pub struct AppError {
  pub status_code: StatusCode,
  pub code: u32,
  pub message: String,
}

impl From<anyhow::Error> for AppError
{
  fn from(error: anyhow::Error) -> AppError {
    AppError {
      status_code: StatusCode::OK,
      code: 1,
      message: error.to_string(),
    }
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    result::fail_status_code(self.status_code, self.code, &self.message).into_response()
  }
}
