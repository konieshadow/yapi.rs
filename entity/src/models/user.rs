use sea_orm::{entity::prelude::*, ConnectionTrait, QuerySelect, FromQueryResult, ItemsAndPagesNumber, QueryOrder};
use serde::{Deserialize, Serialize};
use yapi_common::types::{UserInfo, UserSearch, UserList, PageList};
use yapi_common::traits::Paginator;
use yapi_macros::AutoTimestampModel;

use crate::traits::AutoTimestamp;


#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Serialize, Deserialize, DeriveActiveEnum, AutoTimestampModel)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum UserRole {
    #[sea_orm(string_value = "admin")]
    Admin,

    #[sea_orm(string_value = "member")]
    Member,
}

impl UserRole {
    pub fn is_admin(&self) -> bool {
        self == &Self::Admin
    }
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
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            role: self.role.clone().into_value(),
            user_type: String::from("site"),
            study: self.study,
            add_time: self.add_time,
            up_time: self.up_time,
        }
    }

    pub fn to_user_search(&self) -> UserSearch {
        UserSearch {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            role: self.role.clone().into_value(),
            add_time: self.add_time,
            up_time: self.up_time,
        }
    }
}

impl Entity {
    pub async fn find_user_role_by_id<C>(db: &C, uid: u32) -> Result<Option<UserRole>, DbErr>
    where C: ConnectionTrait
    {
        #[derive(FromQueryResult)]
        struct UserInfo {
            pub role: UserRole,
        }

        Entity::find()
            .select_only()
            .column(Column::Role)
            .filter(
                Column::Id.eq(uid)
            )
            .into_model::<UserInfo>()
            .one(db)
            .await
            .map(|m| {
                m.map(|m| m.role)
            })
    }

    pub async fn find_user_info_by_id<C>(db: &C, uid: u32) -> Result<Option<UserInfo>, DbErr>
    where C: ConnectionTrait
    {
        Entity::find_by_id(uid)
            .one(db)
            .await
            .map(|m| {
                m.map(|m| m.to_user_info())
            })
    }

    pub async fn find_exist_uids<C>(db: &C, uids: &[u32]) -> Result<Vec<u32>, DbErr>
    where C: ConnectionTrait
    {
        if uids.is_empty() {
            return Ok(Vec::new());
        }

        #[derive(FromQueryResult)]
        struct Id {
            id: u32,
        }

        let result = Entity::find()
            .select_only()
            .column(Column::Id)
            .filter(Column::Id.is_in(uids.to_owned()))
            .into_model::<Id>()
            .all(db)
            .await?
            .into_iter()
            .map(|m| m.id)
            .collect();

        Ok(result)
    }

    pub async fn find_user_list_page<C>(db: &C, query: UserList) -> Result<PageList<UserInfo>, DbErr>
    where C: ConnectionTrait
    {
        let ItemsAndPagesNumber {
            number_of_items: count,
            number_of_pages: total,
        } = Entity::find()
            .paginate(db, query.page_size())
            .num_items_and_pages()
            .await?;
    
        let list: Vec<UserInfo> = Entity::find()
            .order_by_desc(Column::Id)
            .paginate(db, query.page_size())
            .fetch_page(query.page())
            .await?
            .into_iter().map(|m| m.to_user_info())
            .collect();
    
        Ok(PageList::new(count, total, list))
    }
}
