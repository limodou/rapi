// use thiserror::Error;
use anyhow;
use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
// use std::fmt;
use super::result;

#[derive(Debug)]
pub struct AppError {
  pub status_code: StatusCode,
  pub code: u32,
  pub message: String,
}

// impl fmt::Display for AppError {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "({}, {}, {})", self.status_code, self.code, self.message)
//   }
// }

// impl Error for AppError {
//   fn description(&self) -> &str {
//     self.message.as_str()
//   }
// }

// impl<E> From<E> for AppError
// where
//   E: std::error::Error,
// {
//   fn from(error: E) -> AppError {
//     AppError {
//       status_code: StatusCode::INTERNAL_SERVER_ERROR,
//       code: 1,
//       message: error.to_string(),
//     }
//   }
// }

impl From<anyhow::Error> for AppError
// where
//   E: std::error::Error,
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

// impl IntoResponse for anyhow::Error {
//   fn into_response(self) -> Response {
//     result::fail_status_code(StatusCode::OK, 999, &self.to_string()).into_response()
//   }
// }
