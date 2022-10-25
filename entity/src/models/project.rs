use sea_orm::entity::prelude::*;

use super::base::TypeVisible;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "project")]
pub struct Model {
    #[sea_orm(primary_key, auto_increament = true)]
    pub id: u32,

    pub uid: u32,

    #[sea_orm(indexed)]
    pub group_id: u32,
    
    pub name: String,
    pub basepath: String,

    #[sea_orm(default_value = true)]
    pub switch_notice: bool,

    pub desc: String,

    #[sea_orm(column_name = "type")]
    project_type: TypeVisible,

    pub icon: String,
    pub color: String,

    #[sea_orm(default_value = false)]
    pub is_mock_opened: bool,

    #[sea_orm(default_value = true)]
    pub is_json5: bool,

    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}