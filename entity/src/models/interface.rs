use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use yapi_common::types::NameValue;
use yapi_macros::AutoTimestampModel;

use crate::traits::AutoTimestamp;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum InterfaceStatus {
    #[sea_orm(string_value = "undone")]
    Undone,

    #[sea_orm(string_value = "done")]
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum ReqBodyType {
    #[sea_orm(string_value = "form")]
    Form,

    #[sea_orm(string_value = "json")]
    Json,

    #[sea_orm(string_value = "text")]
    Text,

    #[sea_orm(string_value = "file")]
    File,

    #[sea_orm(string_value = "raw")]
    Raw,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Serialize, Deserialize, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum FormType {
    #[sea_orm(string_value = "text")]
    Text,

    #[sea_orm(string_value = "file")]
    File,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Serialize, Deserialize, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum ResBodyType {
    #[sea_orm(string_value = "json")]
    Json,

    #[sea_orm(string_value = "text")]
    Text,

    #[sea_orm(string_value = "xml")]
    Xml,

    #[sea_orm(string_value = "raw")]
    Raw,

    #[sea_orm(string_value = "json-schema")]
    JsonSchema,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct QueryPath {
    pub path: String,
    pub params: Vec<NameValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqQuery {
    pub name: String,
    pub value: String,
    pub example: String,
    pub desc: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqQuerys(Vec<ReqQuery>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqHeader {
    pub name: String,
    pub value: String,
    pub example: String,
    pub desc: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqHeaders(Vec<ReqHeader>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqParam {
    pub name: String,
    pub example: String,
    pub desc: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqParams(Vec<ReqParam>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqBodyForm {
    pub name: String,
    pub form_type: FormType,
    pub example: String,
    pub value: String,
    pub desc: String,
    required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqBodyForms(Vec<ReqBodyForm>);

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, AutoTimestampModel)]
#[sea_orm(table_name = "interface")]
pub struct Model {
    #[sea_orm(primary_key, auto_increament = true)]
    pub id: u32,

    pub uid: u32,

    #[sea_orm(indexed)]
    pub project_id: u32,

    pub cat_id: u32,
    pub title: String,
    pub method: String,
    pub path: String,
    pub status: InterfaceStatus,
    pub desc: String,
    pub markdown: String,
    pub query_path: QueryPath,
    pub req_query: ReqQuerys,
    pub req_headers: ReqHeaders,
    pub req_params: ReqParams,
    pub req_body_type: ReqBodyType,

    #[sea_orm(default_value = false)]
    pub req_body_is_json_schema: bool,

    pub req_body_form: ReqBodyForms,
    pub req_body_other: String,
    pub res_body_type: ResBodyType,
    pub res_body: String,

    #[sea_orm(default_value = true)]
    pub res_body_is_json_schema: bool,

    #[sea_orm(default_value = true)]
    pub api_opened: bool,

    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}