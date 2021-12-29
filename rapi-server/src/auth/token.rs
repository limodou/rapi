use chrono::Utc;
use jsonwebtoken as jwt;
use jwt::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
  pub sub: String,
  pub exp: i64,
}

impl Token {
  pub fn new(username: String, exp: i64) -> Self {
    Self {
      sub: username,
      exp: Utc::now().timestamp() + exp,
    }
  }

  #[allow(dead_code)]
  pub fn sign(&self) -> anyhow::Result<String> {
    let mut header = Header::default();
    header.alg = Algorithm::HS512;
    let key = get_jwt_secret();
    let token = encode(&header, &self, &EncodingKey::from_secret(key.as_ref()))?;
    Ok(token)
  }

  #[allow(dead_code)]
  pub fn parse(token: &str) -> anyhow::Result<Self> {
    // let validation = Validation {leeway: 600, ..Default::default()};
    let key = get_jwt_secret();
    let res = decode::<Self>(
      token,
      &DecodingKey::from_secret(key.as_ref()),
      &Validation::new(Algorithm::HS512),
    )
    .map_err(|e| anyhow::anyhow!(format!("1003 {}", e.to_string())))?;
    Ok(res.claims)
  }
}

fn get_jwt_secret() -> String {
  match std::env::var_os("JWT_SECRET") {
    Some(t) => t.to_str().unwrap().to_string(),
    None => panic!("JWT_SECRET is not set in env"),
  }
}
