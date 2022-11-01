use sea_orm::FromQueryResult;
use serde::{Serialize, Deserialize};
use validator::Validate;

use super::MemberInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GroupAdd {
    #[validate(length(min = 2, max = 30, message = "长度必须在2到30之间"))]
    pub group_name: String,

    #[validate(length(max = 500, message = "长度必须在500字符之内"))]
    pub group_desc: String,

    pub owner_uids: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupWithMember {
    pub id: u32,
    pub uid: u32,
    pub group_name: String,
    pub group_desc: String,

    #[serde(rename = "type")]
    pub group_type: String,

    pub member: Vec<MemberInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct GroupInfo {
    pub id: u32,
    pub uid: u32,
    pub group_name: String,
    pub role: String,
    
    #[serde(rename = "type")]
    pub group_type: String,

    pub add_time: u32,
    pub up_time: u32,
}