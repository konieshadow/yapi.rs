use sea_orm::FromQueryResult;
use serde::{Serialize, Deserialize};
use ts_rs::TS;
use validator::Validate;

use super::MemberInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct GroupAdd {
    #[validate(length(min = 1, max = 30, message = "长度必须在1到30之间"))]
    pub group_name: String,

    #[validate(length(max = 500, message = "长度必须在500字符之内"))]
    pub group_desc: String,

    pub owner_uids: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct GroupUp {
    pub id: u32,

    #[validate(length(min = 2, max = 30, message = "长度必须在2到30之间"))]
    pub group_name: Option<String>,

    #[validate(length(max = 500, message = "长度必须在500字符之内"))]
    pub group_desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct GroupWithMember {
    pub id: u32,
    pub uid: u32,
    pub group_name: String,
    pub group_desc: String,

    #[serde(rename = "type")]
    pub group_type: String,

    pub member: Vec<MemberInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct GroupInfo {
    pub id: u32,
    pub uid: u32,
    pub group_name: String,
    pub role: Option<String>,
    
    #[serde(rename = "type")]
    pub group_type: String,

    pub add_time: u32,
    pub up_time: u32,
}