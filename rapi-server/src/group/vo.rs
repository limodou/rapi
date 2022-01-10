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
