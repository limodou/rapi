use super::result;
use lazy_static::lazy_static;
use poem::{error::Error, IntoResponse};
use regex::Regex;

lazy_static! {
  static ref ERROR: regex::Regex = Regex::new(r"^(\d+)\s+(.*)$").unwrap();
}

pub async fn custom_error(err: Error) -> impl IntoResponse {
  let message = err.to_string();
  println!("{}", message);
  match ERROR.captures(&message) {
    Some(m) => result::fail(
      m.get(1).unwrap().as_str().parse().unwrap(),
      m.get(2).unwrap().as_str(),
    ),
    None => result::fail(9999, &message),
  }
}
