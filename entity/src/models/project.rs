use sea_orm::{entity::prelude::*, ConnectionTrait, FromQueryResult, sea_query::{Query, Expr, Alias, Cond}, QueryOrder, QuerySelect};
use yapi_common::{types::{ProjectInfo, ProjectList, ProjectItem, DeleteResult}, traits::Paginator};
use yapi_macros::AutoTimestampModel;

use crate::{base::MemberRole, group_member_entity, project_member_entity, project_env_entity, group_entity, interface_cat_entity, interface_entity};
use crate::traits::AutoTimestamp;

use super::base::TypeVisible;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, AutoTimestampModel)]
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
    pub project_type: TypeVisible,

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

impl Model {
    pub fn to_project_item(self) -> ProjectItem {
        ProjectItem {
            id: self.id,
            uid: self.uid,
            name: self.name,
            basepath: self.basepath,
            switch_notice: self.switch_notice,
            desc: self.desc,
            group_id: self.group_id,
            project_type: self.project_type.into_value(),
            color: self.color,
            icon: self.icon,
            is_json5: self.is_json5,
            is_mock_open: self.is_mock_opened,
            add_time: self.add_time,
            up_time: self.up_time,
        }
    }
}

impl Entity {
    pub async fn find_project_role_by_uid<C>(db: &C, uid: u32, project_id: u32) -> Result<Option<MemberRole>, DbErr>
    where C: ConnectionTrait
    {
        #[derive(FromQueryResult)]
        struct Result {
            group_uid: u32,
            group_type: TypeVisible,
            group_role: Option<MemberRole>,
            project_role: Option<MemberRole>,
        }

        let none_permissoin = 10000;

        let mut stmt = Query::select();
        stmt.expr_as(Expr::col((group_entity::Entity, group_entity::Column::Uid)), Alias::new("group_uid"))
            .expr_as(Expr::col((group_entity::Entity, group_entity::Column::GroupType)), Alias::new("group_type"))
            .expr_as(Expr::col((group_member_entity::Entity, group_member_entity::Column::Role)), Alias::new("group_role"))
            .expr_as(Expr::col((project_member_entity::Entity, project_member_entity::Column::Role)), Alias::new("project_role"))
            .from(Entity)
            .inner_join(group_entity::Entity,
                Expr::tbl(group_entity::Entity, group_entity::Column::Id)
                    .equals(Entity, Column::GroupId)
            )
            .left_join(group_member_entity::Entity,
                Cond::all()
                    .add(Expr::tbl(group_member_entity::Entity, group_member_entity::Column::GroupId)
                        .equals(Entity, Column::GroupId))
                    .add(Expr::tbl(group_member_entity::Entity, group_member_entity::Column::Uid)
                        .eq(uid))
            )
            .left_join(project_member_entity::Entity,
                Cond::all()
                    .add(Expr::tbl(project_member_entity::Entity, project_member_entity::Column::ProjectId)
                        .equals(Entity, Column::Id))
                    .add(Expr::tbl(project_member_entity::Entity, group_member_entity::Column::Uid)
                        .eq(uid))
            )
            .and_where(Column::Id.eq(project_id));
            
        let builder = db.get_database_backend();
        let result = Result::find_by_statement(builder.build(&stmt))
            .one(db)
            .await?;

        if let Some(result) = result {
            // ??????????????????
            if result.group_type == TypeVisible::Private {
                // ????????????
                if result.group_uid == uid {
                    // ????????????
                    Ok(Some(MemberRole::Owner))
                } else {
                    // ???????????????
                    Ok(None)
                }
            } else {
                // ????????????????????????????????????????????????????????????
                let group_role_value = result.group_role.to_owned().map(|m| m as usize).unwrap_or(none_permissoin);
                let project_role_value = result.project_role.to_owned().map(|m| m as usize).unwrap_or(none_permissoin);
                if group_role_value > project_role_value {
                    Ok(result.project_role)
                } else if group_role_value < project_role_value {
                    Ok(result.group_role)
                } else if project_role_value == none_permissoin {
                    Ok(None)
                } else {
                    Ok(result.project_role)
                }
            }
        } else {
            Ok(None)
        }
    }

    pub async fn find_project_info<C>(db: &C, project_id: u32) -> Result<Option<ProjectInfo>, DbErr>
    where C: ConnectionTrait
    {
        let project_item = Entity::find_by_id(project_id)
            .one(db)
            .await?
            .map(|m| m.to_project_item());

        if project_item.is_none() {
            return Ok(None)
        }

        if let Some(project_item) = project_item {
        // ??????????????????
            let env = project_env_entity::Entity::find_project_info(db, project_id).await?;
            Ok(Some(ProjectInfo { project_item, env }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_project_list_by_group<C>(db: &C, query: ProjectList) -> Result<Vec<ProjectItem>, DbErr>
    where C: ConnectionTrait
    {
        let list: Vec<ProjectItem> = Entity::find()
            .filter(Column::GroupId.eq(query.group_id))
            .order_by_desc(Column::Id)
            .paginate(db, query.page_size())
            .fetch_page(query.page())
            .await?
            .into_iter()
            .map(|m| m.to_project_item())
            .collect();

        Ok(list)
    }

    pub async fn find_project_ids_by_group<C>(db: &C, group_id: u32) -> Result<Vec<u32>, DbErr>
    where C: ConnectionTrait
    {
        #[derive(FromQueryResult)]
        struct Result {
            id: u32,
        }

        let ids = Entity::find()
            .select_only()
            .column(Column::Id)
            .filter(Column::GroupId.eq(group_id))
            .into_model::<Result>()
            .all(db)
            .await?
            .into_iter()
            .map(|m| m.id)
            .collect();

        Ok(ids)
    }

    pub async fn delete_project<C>(db: &C, project_id: u32) -> Result<DeleteResult, DbErr>
    where C: ConnectionTrait
    {
        // ????????????
        let result = Entity::delete_many()
            .filter(Column::Id.eq(project_id))
            .exec(db)
            .await?;

        // ??????????????????
        project_env_entity::Entity::delete_many()
            .filter(project_env_entity::Column::ProjectId.eq(project_id))
            .exec(db)
            .await?;

        // ??????????????????
        project_member_entity::Entity::delete_many()
            .filter(project_member_entity::Column::ProjectId.eq(project_id))
            .exec(db)
            .await?;

        // ??????????????????????????????
        interface_cat_entity::Entity::delete_many()
            .filter(interface_cat_entity::Column::ProjectId.eq(project_id))
            .exec(db)
            .await?;

        // ??????????????????????????????
        interface_entity::Entity::delete_many()
            .filter(interface_entity::Column::ProjectId.eq(project_id))
            .exec(db)
            .await?;

        Ok(result.into())
    }
}