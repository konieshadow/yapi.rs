use sea_orm::entity::prelude::*;

use super::base::MemberRole;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "project_member")]
pub struct Model {

    #[sea_orm(primary_key)]
    pub project_id: u32,

    #[sea_orm(primary_key)]
    pub uid: u32,

    pub role: MemberRole,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}