use sea_orm::{entity::prelude::*, ConnectionTrait, sea_query::{Query, Expr}, JoinType, FromQueryResult};
use yapi_common::types::MemberInfo;

use crate::models::base::MemberRole;

use super::user;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "group_member")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub group_id: u32,

    #[sea_orm(primary_key)]
    pub uid: u32,

    pub role: MemberRole,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub async fn find_member_by_group<C>(db: &C, group_id: u32) -> Result<Vec<MemberInfo>, DbErr>
    where C: ConnectionTrait
    {
        let mut stmt = Query::select();
        stmt.columns([
                (Entity, Column::Role),
            ])
            .columns([
                (user::Entity, user::Column::Id),
                (user::Entity, user::Column::Username),
                (user::Entity, user::Column::Email),
                (user::Entity, user::Column::Role)
            ])
            .from(Entity)
            .join(
                JoinType::LeftJoin,
                user::Entity,
                Expr::tbl(user::Entity, user::Column::Id)
                    .equals(Entity, Column::Uid),
            )
            .and_where(Column::GroupId.eq(group_id));

        let builder = db.get_database_backend();
        MemberInfo::find_by_statement(builder.build(&stmt))
            .all(db)
            .await
    }
}