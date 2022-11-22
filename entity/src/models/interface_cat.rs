use sea_orm::{entity::prelude::*, ConnectionTrait};
use yapi_common::types::InterfaceCat;
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

impl Model {
    pub fn to_interface_cat(self) -> InterfaceCat {
        InterfaceCat {
            id: self.id,
            uid: self.uid,
            name: self.name,
            project_id: self.project_id,
            desc: self.desc,
            add_time: self.add_time,
            up_time: self.up_time,
        }
    }
}

impl Entity {
    pub async fn find_interface_cat_by_project<C>(db: &C, project_id: u32) -> Result<Vec<InterfaceCat>, DbErr>
    where C: ConnectionTrait
    {
        let list: Vec<InterfaceCat> = Entity::find()
            .filter(Column::ProjectId.eq(project_id))
            .all(db)
            .await?
            .into_iter()
            .map(|m| m.to_interface_cat())
            .collect();

        Ok(list)
    }
}