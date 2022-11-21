use time::OffsetDateTime;
use sea_orm::{entity::prelude::*, ConnectionTrait, Set};
use yapi_common::types::ProjectEnv;

use crate::base::{NameValueVec, AutoTimestamp};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
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


impl AutoTimestamp for ActiveModel {
    fn default_add() -> Self {
        let timestamp = OffsetDateTime::now_utc().unix_timestamp() as u32;
        Self {
            add_time: Set(timestamp),
            up_time: Set(timestamp),
            ..Default::default()
        }
    }

    fn default_up() -> Self {
        let timestamp = OffsetDateTime::now_utc().unix_timestamp() as u32;
        Self {
            up_time: Set(timestamp),
            ..Default::default()
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