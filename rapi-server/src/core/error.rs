// use thiserror::Error;
use anyhow;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
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

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.status_code, self.code, &self.message)
    }
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
