mod user;
mod group;

use sea_orm::{FromQueryResult};
use serde::{Serialize, Deserialize};
pub use user::*;
pub use group::*;
use validator::{Validate, ValidationError};

use crate::utils::validator::valid_one_of;

fn page_default() -> usize {
    1
}

fn limit_default() -> usize {
    20
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginator {
    #[serde(default = "page_default")]
    page: usize,

    #[serde(default = "limit_default")]
    limit: usize,
}

impl Paginator {
    pub fn page_size(&self) -> usize {
        self.limit
    }

    pub fn page(&self) -> usize {
        if self.page == 0 {
            0
        } else {
            self.page - 1
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageList<T> {
    pub count: usize,
    pub total: usize,
    pub list: Vec<T>,
}

impl <T> PageList<T> {
    pub fn new(count: usize, total: usize, list: Vec<T>) -> Self {
        PageList { count, total, list }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    pub modified_count: u32,
}

impl From<sea_orm::UpdateResult> for UpdateResult {
    fn from(r: sea_orm::UpdateResult) -> Self {
        Self { modified_count: r.rows_affected as u32 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResult {
    pub deleted_count: u32,
}

impl From<sea_orm::DeleteResult> for DeleteResult {
    fn from(r: sea_orm::DeleteResult) -> Self {
        Self { deleted_count: r.rows_affected as u32 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search {
    pub q: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetById {
    pub id: u32,
}

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