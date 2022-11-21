use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use yapi_common::types::NameValue;

pub trait AutoTimestamp {
    fn default_add() -> Self;
    fn default_up() -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Serialize, Deserialize, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum TypeVisible {
    #[sea_orm(string_value = "public")]
    Public,

    #[sea_orm(string_value = "private")]
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Serialize, Deserialize, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum MemberRole {
    #[sea_orm(string_value = "owner")]
    Owner = 0,

    #[sea_orm(string_value = "dev")]
    Dev = 10,

    #[sea_orm(string_value = "guest")]
    Guest = 20,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, FromJsonQueryResult)]
pub struct NameValueVec(pub Vec<NameValue>);
