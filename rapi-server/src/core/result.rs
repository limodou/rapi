use poem::{http::StatusCode, IntoResponse, web::Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Res<T> {
  pub code: u32,
  pub result: Option<T>,
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
pub fn ok_data<T>(data: T) -> impl IntoResponse
where
  T: serde::Serialize,
{
  let res = Res {
    code: 0,
    result: Some(data),
    message: None,
  };
  let body = Json(json!(res));
  (StatusCode::OK, body).into_response()
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
