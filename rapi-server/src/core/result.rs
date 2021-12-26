use axum::{http::StatusCode, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Res<T> {
  pub code: u32,
  pub data: Option<T>,
  pub message: Option<String>,
}

#[allow(dead_code)]
pub fn ok_message(message: &str) -> impl IntoResponse {
  let body = Json(json!({
      "code": 0,
      "message": message,
  }));

  (StatusCode::OK, body).into_response()
}

#[allow(dead_code)]
pub fn ok() -> StatusCode {
  StatusCode::OK
}

#[allow(dead_code)]
pub fn fail(code: u32, message: &str) -> impl IntoResponse {
  let body = Json(json!({
      "code": code,
      "message": message,
  }));

  (StatusCode::OK, body).into_response()
}

#[allow(dead_code)]
pub fn fail_status_code(status_code: StatusCode, code: u32, message: &str) -> impl IntoResponse {
  let body = Json(json!({
      "code": code,
      "message": message,
  }));

  (status_code, body).into_response()
}
