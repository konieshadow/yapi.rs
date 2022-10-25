use sea_orm::{entity::prelude::*, ConnectionTrait};
use serde::{Deserialize, Serialize};
use yapi_common::types::UserInfo;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Serialize, Deserialize, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum UserRole {
    #[sea_orm(string_value = "Admin")]
    Admin,

    #[sea_orm(string_value = "Member")]
    Member,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increament = true)]
    pub id: u32,

    #[sea_orm(unique)]
    pub username: String,

    #[sea_orm(unique)]
    pub email: String,

    pub password: String,

    #[sea_orm(default_value = false)]
    pub study: bool,

    pub role: UserRole,
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            uid: self.id,
            username: self.username.to_owned(),
            email: self.email.to_owned(),
            role: self.role.to_owned().into_value(),
            user_type: "site".to_owned(),
            study: self.study,
            add_time: self.add_time,
            up_time: self.up_time,
        }
    }
}

impl Entity {

    pub async fn find_user_info_by_id<C>(db: &C, id: u32) -> Result<Option<UserInfo>, DbErr>
    where C: ConnectionTrait
    {
        Entity::find_by_id(id)
            .one(db)
            .await
            .map(|m| {
                m.map(|m| m.to_user_info())
            })
    }
}
