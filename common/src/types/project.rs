use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::{Validate, ValidationError};
use yapi_macros::PaginatorQuery;

use crate::{utils::validator::valid_one_of, traits::Paginator};

use super::{NameValue, InterfaceCat};

fn valid_project_type_fn(value: &str) -> Result<(), ValidationError> {
    valid_one_of(value, vec!["public", "private"])
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ProjectAdd {
    #[validate(length(min = 1, max = 30, message = "长度必须在1到30之间"))]
    pub name: String,

    pub group_id: u32,

    #[validate(length(max = 100, message = "长度不能大于100"))]
    pub basepath: String,

    #[validate(length(max = 200, message = "长度不能大于200"))]
    pub desc: String,

    #[validate(custom = "valid_project_type_fn")]
    pub project_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ProjectUp {
    pub id: u32,
    
    #[validate(length(min = 1, max = 30, message = "长度必须在1到30之间"))]
    pub name: Option<String>,

    pub group_id: Option<u32>,

    #[validate(length(max = 100, message = "长度不能大于100"))]
    pub basepath: Option<String>,

    #[validate(length(max = 200, message = "长度不能大于200"))]
    pub desc: Option<String>,

    pub is_json5: Option<bool>,
    pub switch_notice: Option<bool>,

    #[validate(custom = "valid_project_type_fn")]
    pub project_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PaginatorQuery, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ProjectList {
    pub group_id: u32,
    page: Option<usize>,
    limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ProjectItem {
    pub id: u32,
    pub uid: u32,
    pub name: String,
    pub basepath: String,
    pub switch_notice: bool,
    pub desc: String,
    pub group_id: u32,
    pub project_type: String,
    pub color: String,
    pub icon: String,
    pub is_json5: bool,
    pub is_mock_open: bool,
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ProjectInfo {
    #[serde(flatten)]
    pub project_item: ProjectItem,

    pub env: Vec<ProjectEnv>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ProjectEnv {
    pub id: u32,
    pub name: String,
    pub domain: String,
    pub header: Vec<NameValue>,
    pub global: Vec<NameValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ProjectDetail {
    #[serde(flatten)]
    pub project_info: ProjectInfo,

    pub cat: Vec<InterfaceCat>,
}