use sea_orm::{entity::prelude::*, ConnectionTrait, sea_query::{Query, Expr, Alias, Cond}, FromQueryResult, Order, Condition};
use yapi_common::types::GroupInfo;
use yapi_macros::AutoTimestampModel;

use super::{base::TypeVisible, group_member_entity};
use crate::traits::AutoTimestamp;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, AutoTimestampModel)]
#[sea_orm(table_name = "group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increament = true)]
    pub id: u32,

    pub uid: u32,

    pub group_name: String,

    pub group_desc: String,

    #[sea_orm(column_name = "type")]
    pub group_type: TypeVisible,

    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub async fn find_group_info<C>(db: &C, uid: u32, group_id: u32) -> Result<Option<GroupInfo>, DbErr>
    where C: ConnectionTrait
    {
        let mut stmt = Query::select();
        stmt.columns([
                (Entity, Column::Id),
                (Entity, Column::Uid),
                (Entity, Column::GroupName),
                (Entity, Column::AddTime),
                (Entity, Column::UpTime),
            ])
            .expr_as(Expr::col((Entity, Column::GroupType)), Alias::new("group_type"))
            .column((group_member_entity::Entity, group_member_entity::Column::Role))
            .from(Entity)
            .left_join(group_member_entity::Entity,
                Expr::tbl(group_member_entity::Entity, group_member_entity::Column::GroupId)
                    .equals(Entity, Column::Id)
            )
            .and_where(Column::Id.eq(group_id))
            .cond_where(
                Cond::any()
                    .add(group_member_entity::Column::Uid.eq(uid))
                    .add(
                        Cond::all()
                            .add(Column::Uid.eq(uid))
                            .add(Column::GroupType.eq(TypeVisible::Private))
                    )
            );

        let builder = db.get_database_backend();
        GroupInfo::find_by_statement(builder.build(&stmt))
            .one(db)
            .await
    }

    pub async fn find_group_list<C>(db: &C, uid: u32) -> Result<Vec<GroupInfo>, DbErr>
    where C: ConnectionTrait
    {
        let mut stmt = Query::select();
        stmt.columns([
                (Entity, Column::Id),
                (Entity, Column::Uid),
                (Entity, Column::GroupName),
                (Entity, Column::AddTime),
                (Entity, Column::UpTime),
            ])
            .expr_as(Expr::col((Entity, Column::GroupType)), Alias::new("group_type"))
            .column((group_member_entity::Entity, group_member_entity::Column::Role))
            .from(Entity)
            .left_join(group_member_entity::Entity,
                Expr::tbl(group_member_entity::Entity, group_member_entity::Column::GroupId)
                    .equals(Entity, Column::Id)
            )
            .and_where(group_member_entity::Column::Uid.eq(uid))
            .order_by((Entity, Column::Id), Order::Desc);

        let builder = db.get_database_backend();
        GroupInfo::find_by_statement(builder.build(&stmt))
            .all(db)
            .await
    }

    pub async fn find_private_group<C>(db: &C, uid: u32) -> Result<Option<GroupInfo>, DbErr>
    where C: ConnectionTrait
    {
        Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Uid.eq(uid))
                    .add(Column::GroupType.eq(TypeVisible::Private))
            )
            .one(db)
            .await
            .map(|o| o.map(|r| GroupInfo {
                id: r.id,
                uid: r.uid,
                group_name: r.group_name,
                role: None,
                group_type: r.group_type.into_value(),
                add_time: r.add_time,
                up_time: r.up_time,
            }))
    }
}