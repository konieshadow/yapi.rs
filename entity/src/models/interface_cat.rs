use sea_orm::{entity::prelude::*, ConnectionTrait, sea_query::{Query, Expr, Alias}, FromQueryResult};
use yapi_common::types::InterfaceCat;
use yapi_macros::AutoTimestampModel;

use crate::traits::AutoTimestamp;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, AutoTimestampModel)]
#[sea_orm(table_name = "interface_cat")]
pub struct Model {
    #[sea_orm(primary_key, auto_increament = true)]
    pub id: u32,

    pub uid: u32,

    #[sea_orm(indexed)]
    pub project_id: u32,

    pub index: u32,

    pub name: String,

    pub desc: String,

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
            index: self.index,
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

    pub async fn find_max_interface_cat_index<C>(db: &C, project_id: u32) -> Result<u32, DbErr>
    where C: ConnectionTrait
    {
        #[derive(FromQueryResult)]
        struct Result {
            index: Option<u32>,
        }

        let mut stmt = Query::select();
        stmt.expr_as(Expr::col(Column::Index).max(), Alias::new("index"))
            .from(Entity)
            .and_where(Column::ProjectId.eq(project_id));

        let builder = db.get_database_backend();
        let result = Result::find_by_statement(builder.build(&stmt))
            .one(db)
            .await?;

        Ok(result.and_then(|m| m.index).unwrap_or(0))
    }

    pub async fn update_interface_cat_index_after<C>(db: &C, project_id: u32, cat_index: u32) -> Result<(), DbErr>
    where C: ConnectionTrait
    {
        Entity::update_many()
            .filter(
                Column::ProjectId.eq(project_id)
                    .and(Column::Index.gt(cat_index))
            )
            .col_expr(Column::Index, Expr::col(Column::Index).sub(1))
            .exec(db)
            .await?;

        Ok(())
    }

    pub async fn find_project_ids_by_interface_cat_ids<C>(db: &C, cat_ids: &[u32]) -> Result<Vec<u32>, DbErr>
    where C: ConnectionTrait
    {
        #[derive(FromQueryResult)]
        struct Result {
            project_id: u32,
        }

        let mut stmt = Query::select();
        stmt.column(Column::ProjectId)
            .distinct()
            .from(Entity)
            .and_where(Column::Id.is_in(cat_ids.to_owned()));

        let builder = db.get_database_backend();
        let list = Result::find_by_statement(builder.build(&stmt))
            .all(db)
            .await?
            .into_iter()
            .map(|m| m.project_id)
            .collect();

        Ok(list)
    }
}