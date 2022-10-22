use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, EnumIter, Serialize, Deserialize, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum TypeVisible {
    #[sea_orm(string_value = "public")]
    Public,

    #[sea_orm(string_value = "private")]
    Private,
}

#[derive(Debug, Clone, PartialEq, EnumIter, Serialize, Deserialize, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum MemberRole {
    #[sea_orm(string_value = "Owner")]
    Owner,

    #[sea_orm(string_value = "Dev")]
    Dev,

    #[sea_orm(string_value = "Visitor")]
    Visitor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct NameValue {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct NameValueVec(Vec<NameValue>);