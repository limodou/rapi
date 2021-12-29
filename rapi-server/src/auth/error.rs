use anyhow::{anyhow, Error};
// use poem::{error::ResponseError, http::StatusCode};
// #[derive(thiserror::Error, Debug, Copy, Clone, Eq, PartialEq)]
// #[error("Invalid password error")]
// pub struct InvalidPasswordError;

// impl ResponseError for InvalidPasswordError {
//     fn status(&self) -> StatusCode {
//         StatusCode::INTERNAL_SERVER_ERROR
//     }
// }

pub enum AuthErrorKind {
  InvalidPasswordError,
}

pub struct AuthError;

impl AuthError {
  pub fn new(kind: AuthErrorKind) -> Error {
    match kind {
      AuthErrorKind::InvalidPasswordError => anyhow!("1001 Invalid password"),
    }
  }
}