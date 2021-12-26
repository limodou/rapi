use super::vo::RegisterReq;
use anyhow::Result;
use chrono::prelude::*;
use sqlx::{self, mysql::MySqlPool};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub realname: String,
  pub password: String,
  pub access_token: String,
  pub is_super: bool,
  pub error_times: i32,
  pub freeze_time: Option<DateTime<Local>>,
  pub is_available: bool,
  pub is_first_time_login: bool,
  pub last_landing_time: DateTime<Local>,
  pub create_time: DateTime<Local>,
  pub update_time: DateTime<Local>,
  pub version: i32,
}

impl User {
  pub async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<User> {
    let rec= sqlx::query_as::<_, User>(
      r#"SELECT id, username, realname, password, access_token,
    is_super, error_times, freeze_time, is_available, is_first_time_login, last_landing_time,
    create_time, update_time, version
    FROM user WHERE username=?"#,
    )
    .bind(username)
    .fetch_one(pool)
    .await?;
    Ok(rec)
  }
  pub async fn create(pool: &MySqlPool, user: &RegisterReq) -> Result<()> {
    let rec = sqlx::query("INSERT INTO user (username, realname, password) values (?, ?, ?)")
      .bind(&user.username)
      .bind(&user.realname)
      .bind(&user.password)
      .execute(pool)
      .await?
      .last_insert_id();
    println!("last_insert_id={}", rec);
    Ok(())
  }
}
