mod user;
mod group;
mod project;
mod interface;
mod role_permission;

use sea_orm::FromJsonQueryResult;
use serde::{Serialize, Deserialize};

use ts_rs::TS;
pub use user::*;
pub use group::*;
pub use project::*;
pub use interface::*;
pub use role_permission::*;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct List<T> {
    pub list: Vec<T>,
}

impl <T> List<T> {
    pub fn new(list: Vec<T>) -> Self {
        Self { list }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct UpdateResult {
    pub modified_count: u32,
}

impl UpdateResult {
    pub fn of(count: u32) -> Self {
        Self { modified_count: count }
    }
}

impl From<sea_orm::UpdateResult> for UpdateResult {
    fn from(r: sea_orm::UpdateResult) -> Self {
        Self { modified_count: r.rows_affected as u32 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct DeleteResult {
    pub deleted_count: u32,
}

impl DeleteResult {
    pub fn of(count: u32) -> Self {
        Self { deleted_count: count }
    }
}

impl From<sea_orm::DeleteResult> for DeleteResult {
    fn from(r: sea_orm::DeleteResult) -> Self {
        Self { deleted_count: r.rows_affected as u32 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct Search {
    pub q: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct GetById {
    pub id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, FromJsonQueryResult, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct NameValue {
    pub name: String,
    pub value: String,
}