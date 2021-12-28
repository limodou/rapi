use anyhow;
use poem::{Endpoint, Middleware, Request, Result};
use thiserror;
// use anyhow::{anyhow};
pub struct JwtTokenMiddleware;

impl<E: Endpoint> Middleware<E> for JwtTokenMiddleware {
  type Output = JwtTokenMiddlewareImpl<E>;

  fn transform(&self, ep: E) -> Self::Output {
    JwtTokenMiddlewareImpl { ep }
  }
}

/// The new endpoint type generated by the TokenMiddleware.
pub struct JwtTokenMiddlewareImpl<E> {
  ep: E,
}

const TOKEN_HEADER: &str = "token";

#[derive(Debug)]
pub struct User(pub Option<String>);

#[poem::async_trait]
impl<E: Endpoint> Endpoint for JwtTokenMiddlewareImpl<E> {
  type Output = E::Output;

  async fn call(&self, mut req: Request) -> Result<Self::Output> {
    let value = match req
      .headers()
      .get(TOKEN_HEADER)
      .and_then(|value| Some(value.to_str().unwrap().to_string()))
    {
      Some(token) => {
        let t = Token::parse("hello", &token)?;
        Some(t.sub)
      }
      _ => None,
    };
    req.extensions_mut().insert(User(value));

    // call the inner endpoint.
    self.ep.call(req).await
  }
}

use jsonwebtoken as jwt;
use jwt::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
  pub sub: String,
  pub exp: i64,
}

impl Token {
  #[allow(dead_code)]
  pub fn sign(&self, key: &str) -> anyhow::Result<String> {
    let mut header = Header::default();
    header.alg = Algorithm::HS512;
    let token = encode(&header, &self, &EncodingKey::from_secret(key.as_ref()))?;
    Ok(token)
  }

  #[allow(dead_code)]
  pub fn parse(key: &str, token: &str) -> anyhow::Result<Self> {
    // let validation = Validation {leeway: 600, ..Default::default()};
    let res = decode::<Self>(
      token,
      &DecodingKey::from_secret(key.as_ref()),
      &Validation::new(Algorithm::HS512),
    )?;
    Ok(res.claims)
  }
}

use poem::{error::ResponseError, http::StatusCode};
#[derive(thiserror::Error, Debug, Copy, Clone, Eq, PartialEq)]
#[error("Token parsed error")]
pub struct TokenParseError;

impl ResponseError for TokenParseError {
  fn status(&self) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
  }
}
