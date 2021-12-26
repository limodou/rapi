use anyhow::Result;
use jsonwebtoken as jwt;
use jwt::{decode, encode, Algorithm, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
  pub sub: String,
  pub exp: usize,
}

impl Token {
  pub fn sign(&self, key: &str) -> Result<String> {
    let mut header = Header::default();
    header.alg = Algorithm::HS512;
    let token = encode(&header, &self, &EncodingKey::from_secret(key.as_ref()))?;
    Ok(token)
  }
}
