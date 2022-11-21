use sea_orm::entity::prelude::*;
use yapi_macros::AutoTimestampModel;

use crate::traits::AutoTimestamp;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, AutoTimestampModel)]
#[sea_orm(table_name = "interface_cat")]
pub struct Model {
    #[sea_orm(primary_key, auto_increament = true)]
    pub id: u32,

    pub uid: u32,

    pub name: String,

    pub desc: String,

    #[sea_orm(indexed)]
    pub project_id: u32,

    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}