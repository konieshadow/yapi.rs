use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

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
    Owner,

    #[sea_orm(string_value = "dev")]
    Dev,

    #[sea_orm(string_value = "visitor")]
    Visitor,
}

impl MemberRole {
    pub fn try_from_str(string_value: &str) -> Option<Self> {
        if string_value == Self::Owner.into_value().as_str() {
            Some(Self::Owner)
        } else if string_value == Self::Dev.into_value().as_str() {
            Some(Self::Dev)
        } else if string_value == Self::Dev.into_value().as_str() {
            Some(Self::Visitor)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct NameValue {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct NameValueVec(Vec<NameValue>);