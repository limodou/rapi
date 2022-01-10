use super::vo::*;
use anyhow::Result;
use sqlx::{self, mysql::MySqlConnection};

#[derive(Debug, sqlx::FromRow)]
pub struct Group {
    pub id: i32,
    pub group_title: String,
    pub group_desc: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GroupUser {
    pub id: i32,
    pub group_id: i32,
    pub user_id: i32,
    pub member_type: String,
}

impl Group {
    pub async fn create(pool: &mut MySqlConnection, group: &CreateGroupReq) -> Result<Group> {
        let rec = sqlx::query("INSERT INTO `group` (group_title, group_desc) values (?, ?)")
            .bind(&group.group_title)
            .bind(&group.group_desc)
            .execute(&mut *pool)
            .await?
            .last_insert_id();
        let row = Group {
            id: rec as i32,
            group_title: group.group_title.clone(),
            group_desc: group.group_desc.clone(),
        };
        if let Some(leader) = group.group_leader {
            row.add_user(&mut *pool, leader, "1").await?;
        }
        Ok(row)
    }

    #[allow(dead_code)]
    pub async fn find_by_title(pool: &mut MySqlConnection, title: &str) -> Result<Option<Group>> {
        let group = sqlx::query_as::<_, Group>(
            "SELECT id, group_title, group_desc FROM `group` WHERE group_title = ?",
        )
        .bind(title)
        .fetch_optional(pool)
        .await?;
        Ok(group)
    }

    // 添加用户信息
    #[allow(dead_code)]
    pub async fn add_user(
        &self,
        pool: &mut MySqlConnection,
        user_id: i32,
        member_type: &str,
    ) -> Result<GroupUser> {
        let rec = sqlx::query(
            "INSERT INTO `group_users` (group_id, user_id, member_type) VALUES (?, ?, ?)",
        )
        .bind(self.id)
        .bind(user_id)
        .bind(member_type)
        .execute(pool)
        .await?
        .last_insert_id();
        let row = GroupUser {
            id: rec as i32,
            group_id: self.id,
            user_id,
            member_type: member_type.to_string(),
        };
        Ok(row)
    }
}
