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
    Owner = 0,

    #[sea_orm(string_value = "dev")]
    Dev = 10,

    #[sea_orm(string_value = "guest")]
    Guest = 20,
}

impl MemberRole {
    pub fn try_from_str(string_value: &str) -> Option<Self> {
        if string_value == "owner" {
            Some(Self::Owner)
        } else if string_value ==  "dev" {
            Some(Self::Dev)
        } else if string_value == "guest" {
            Some(Self::Guest)
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