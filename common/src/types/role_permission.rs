use sea_orm::FromQueryResult;
use serde::{Serialize, Deserialize};
use validator::{ValidationError, Validate};

use crate::utils::validator::valid_one_of;

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct MemberInfo {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMemberResult {
    pub add_members: Vec<MemberInfo>,
    pub exist_members: Vec<MemberInfo>,
    pub no_members: Vec<u32>,
}

fn valid_role_fn(value: &str) -> Result<(), ValidationError> {
    valid_one_of(value, vec!["owner", "dev", "guest"])
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AddMember {
    pub id: u32,

    #[validate(length(min = 1))]
    pub member_uids: Vec<u32>,

    #[validate(custom = "valid_role_fn")]
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DeleteMember {
    pub id: u32,
    pub member_uid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ChangeMemberRole {
    pub id: u32,

    pub member_uid: u32,

    #[validate(custom = "valid_role_fn")]
    pub role: String,
}