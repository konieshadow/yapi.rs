use sea_orm::entity::prelude::*;

use super::base::TypeVisible;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increament = true)]
    pub id: u32,

    pub uid: u32,

    #[sea_orm(unique)]
    pub group_name: String,

    pub group_desc: String,

    #[sea_orm(column_name = "type", default_value = TypeVisible::Public)]
    pub group_type: TypeVisible,

    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}