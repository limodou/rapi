use crate::core::AppError;
use axum::http::StatusCode;

pub struct AuthError;

impl AuthError {
  pub fn invalid_password() -> AppError {
    AppError {
      status_code: StatusCode::OK,
      code: 1001,
      message: "密码不正确".to_string(),
    }
  }
}
