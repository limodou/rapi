use super::model::*;
use super::vo::*;
use anyhow::{anyhow, Result};
use sqlx::{self, mysql::MySqlConnection};

#[allow(dead_code)]
pub async fn create(pool: &mut MySqlConnection, group: &CreateGroupReq) -> Result<CreateGroupRes> {
    // 获取组信息
    let g = Group::find_by_title(pool, &group.group_title).await?;
    if g.is_none() {
        let group = Group::create(pool, group).await?;
        Ok(CreateGroupRes {
            id: group.id,
            group_title: group.group_title.clone(),
            group_desc: group.group_desc.clone(),
        })
    } else {
        Err(anyhow::anyhow!("1011 组名已经存在"))
    }
}
