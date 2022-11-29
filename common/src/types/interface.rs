use sea_orm::{FromJsonQueryResult, FromQueryResult};
use serde::{Serialize, Deserialize};
use ts_rs::TS;
use validator::{Validate, ValidationError};
use yapi_macros::PaginatorQuery;

use crate::{traits::Paginator, utils::validator::valid_one_of};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ReqParam {
    pub name: String,
    pub example: String,
    pub desc: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ReqQuery {
    pub name: String,
    pub value: String,
    pub example: String,
    pub desc: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ReqHeader {
    pub name: String,
    pub value: String,
    pub example: String,
    pub desc: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct ReqBodyForm {
    pub name: String,

    #[validate(custom = "valid_req_form_type_fn")]
    pub form_type: String,

    pub example: String,
    pub value: String,
    pub desc: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct InterfaceCat {
    pub id: u32,
    pub uid: u32,
    pub index: u32,
    pub name: String,
    pub project_id: u32,
    pub desc: String,
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct InterfaceCatAdd {
    #[validate(length(min = 1, max = 30, message = "长度必须在1到30之间"))]
    pub name: String,

    pub project_id: u32,

    #[validate(length(max = 200, message = "长度不能大于200"))]
    pub desc: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct InterfaceCatUp {
    pub id: u32,

    #[validate(length(min = 1, max = 30, message = "长度必须在1到30之间"))]
    pub name: Option<String>,

    #[validate(length(max = 200, message = "长度不能大于200"))]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct IndexItem {
    pub id: u32,
    pub index: u32,
}

fn valid_interface_method_fn(value: &str) -> Result<(), ValidationError> {
    valid_one_of(value, vec!["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"])
}

fn valid_interface_status_fn(value: &str) -> Result<(), ValidationError> {
    valid_one_of(value, vec!["undone", "done"])
}

fn valid_req_body_type_fn(value: &str) -> Result<(), ValidationError> {
    valid_one_of(value, vec!["form", "json", "text", "file", "raw"])
}

fn valid_req_form_type_fn(value: &str) -> Result<(), ValidationError> {
    valid_one_of(value, vec!["text", "file"])
}

fn valid_res_body_type_fn(value: &str) -> Result<(), ValidationError> {
    valid_one_of(value, vec!["json", "text", "xml", "raw", "json-schema"])
}


#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct InterfaceAdd {
    pub cat_id: u32,

    #[validate(length(min = 1, max = 50, message = "长度必须在1到50之间"))]
    pub title: String,

    #[validate(custom = "valid_interface_method_fn")]
    pub method: String,

    #[validate(length(max = 200, message = "长度不能大于200"))]
    pub path: String,

}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct InterfaceUp {
    pub id: u32,

    #[validate(length(min = 1, max = 50, message = "长度必须在1到50之间"))]
    pub title: Option<String>,
    
    pub cat_id: Option<u32>,

    #[validate(custom = "valid_interface_method_fn")]
    pub method: Option<String>,

    #[validate(length(max = 200, message = "长度不能大于200"))]
    pub path: Option<String>,

    pub tag: Option<Vec<String>>,

    #[validate(custom = "valid_interface_status_fn")]
    pub status: Option<String>,

    pub desc: Option<String>,
    pub markdown: Option<String>,
    pub req_params: Option<Vec<ReqParam>>,
    pub req_headers: Option<Vec<ReqHeader>>,
    pub req_query: Option<Vec<ReqQuery>>,

    #[validate(custom = "valid_req_body_type_fn")]
    pub req_body_type: Option<String>,

    pub req_body_is_json_schema: Option<bool>,

    #[validate]
    pub req_body_form: Option<Vec<ReqBodyForm>>,

    pub req_body_other: Option<String>,

    #[validate(custom = "valid_res_body_type_fn")]
    pub res_body_type: Option<String>,

    pub res_body_is_json_schema: Option<bool>,
    pub res_body: Option<String>,
    pub api_opened: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct InterfaceDetail {
    pub id: u32,
    pub uid: u32,
    pub cat_id: u32,
    pub project_id: u32,
    pub title: String,
    pub method: String,
    pub path: String,
    pub status: String,
    pub api_opened: bool,
    pub desc: String,
    pub markdown: String,
    pub req_params: Vec<ReqParam>,
    pub req_header: Vec<ReqHeader>,
    pub req_query: Vec<ReqQuery>,
    pub req_body_type: String,
    pub req_body_is_json_schema: bool,
    pub req_body_form: Vec<ReqBodyForm>,
    pub req_body_other: String,
    pub res_body_type: String,
    pub res_body_is_json_schema: bool,
    pub res_body: String,
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct InterfaceMenu {
    pub id: u32,
    pub uid: u32,
    pub project_id: u32,
    pub name: String,
    pub list: Vec<InterfaceInfo>,
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct InterfaceInfo {
    pub id: u32,
    pub uid: u32,
    pub cat_id: u32,
    pub project_id: u32,
    pub title: String,
    pub method: String,
    pub path: String,
    pub status: String,
    pub api_opened: bool,
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PaginatorQuery, TS)]
#[ts(export, export_to = "../client/src/types/")]
pub struct InterfaceList {
    pub id: u32,
    page: Option<usize>,
    limit: Option<usize>,
}