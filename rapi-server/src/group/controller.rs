use super::service;
use super::vo;
use crate::auth::jwt_token::JwtUser;
use crate::core::result;
use anyhow::{anyhow, Result};
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};
use sqlx::{self, mysql::MySqlPool};
use validator::Validate;

#[handler]
pub async fn create_group(
    Json(group): Json<vo::CreateGroupReq>,
    JwtUser(user): JwtUser,
    Data(pool): Data<&MySqlPool>,
) -> Result<impl IntoResponse> {
    group
        .validate()
        .map_err(|e| anyhow!(format!("1011 {}", e.to_string())))?;
    let mut tr = pool.begin().await?;
    let res = service::create(&mut tr, &group).await?;
    tr.commit().await?;
    Ok(result::ok_data(res))
}

#[handler]
pub async fn get_users(
    Json(group): Json<vo::FindGroupUsersReq>,
    JwtUser(user): JwtUser,
    Data(pool): Data<&MySqlPool>,
) -> Result<impl IntoResponse> {
    // group
    //     .validate()
    //     .map_err(|e| anyhow!(format!("1011 {}", e.to_string())))?;
    // let mut tr = pool.begin().await?;
    // let res = service::create(&mut tr, &group).await?;
    // tr.commit().await?;
    let res = service::get_users(&mut *pool.acquire().await?, &group).await?;
    Ok(result::ok_data(res))
}
