use regex::Regex;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;
use yapi_macros::PaginatorQuery;

use crate::traits::Paginator;

lazy_static! {
    static ref RE_PASSWORD: Regex = Regex::new(r"[A-Za-z\d$@$!%*#?&]{8,20}").unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct UserInfo {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub role: String,

    #[serde(rename = "type")]
    pub user_type: String,

    pub study: bool,
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct AuthUserInfo {
    #[serde(flatten)]
    pub user_info: UserInfo,

    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct UserSearch {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PaginatorQuery, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct UserList {
    page: Option<usize>,
    limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct UserReg {
    #[validate(length(min = 2, max = 30, message = "长度必须在2到30之间"))]
    pub username: String,

    #[validate(length(max = 50, message = "长度最大50位"), email(message = "必须是合法的邮箱地址"))]
    pub email: String,

    #[validate(regex(path = "RE_PASSWORD", message = "必须是8到20位的数字、字母与英文符号的组合"))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct UserLogin {
    #[validate(length(max = 50, message = "长度最大50位"), email(message = "必须是合法的邮箱地址"))]
    pub email: String,

    #[validate(regex(path = "RE_PASSWORD", message = "必须是8到20位的数字、字母与英文符号的组合"))]
    pub password: String,
}