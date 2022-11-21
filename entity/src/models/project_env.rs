use sea_orm::{entity::prelude::*, ConnectionTrait};
use yapi_common::types::ProjectEnv;
use yapi_macros::AutoTimestampModel;

use crate::base::NameValueVec;
use crate::traits::AutoTimestamp;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, AutoTimestampModel)]
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
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_project_env(self) -> ProjectEnv {
        ProjectEnv {
            id: self.id,
            name: self.name,
            domain: self.domain,
            header: self.header.0,
            global: self.global.0,
        }
    }
}

impl Entity {
    pub async fn find_project_info<C>(db: &C, project_id: u32) -> Result<Vec<ProjectEnv>, DbErr>
    where C: ConnectionTrait
    {
        let result: Vec<ProjectEnv> = Entity::find()
            .filter(
                Column::ProjectId.eq(project_id)
            )
            .all(db)
            .await?
            .into_iter()
            .map(|m| m.to_project_env())
            .collect();

        Ok(result)
    }
}