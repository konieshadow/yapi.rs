use sea_orm::entity::prelude::*;

use crate::base::{MemberRole, NameValueVec};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "project_env")]
pub struct Model {

    #[sea_orm(primary_key, auto_increament = true)]
    pub id: u32,

    #[sea_orm(indexed)]
    pub project_id: u32,
    
    pub name: String,
    pub domain: String,
    pub header: NameValueVec,
    pub global: NameValueVec,
    pub role: MemberRole,
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}