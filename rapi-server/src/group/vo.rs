use serde::{Deserialize, Serialize};
use validator::{Validate};

#[derive(Debug, Validate, Deserialize)]
pub struct CreateGroupReq {
  #[validate(length(min=1, message="标题不能为空"))]
  pub group_title: String,
  pub group_desc: String,
  pub group_leader: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CreateGroupRes {
  pub id: i32,
  pub group_title: String,
  pub group_desc: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct FindGroupUsersReq {
  pub group_id: i32,
}

#[derive(Debug, Serialize)]
pub struct FindGroupUsersRes {
  pub users: Vec<GroupUserRes>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct GroupUserRes {
  pub id: i32,
  pub realname: String,
  pub member_type: String,
}
