use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
    QuerySelect,
};
use yapi_entity::{
    base::{MemberRole, TypeVisible},
    group_entity, group_member_entity, project_entity,
    user_entity::{self, UserRole},
};

use crate::{error::Error, Result};

#[derive(Debug, Clone)]
pub enum ActionType {
    Admin = 0,
    Danger = 10,
    Edit = 20,
    View = 30,
}

#[derive(Debug, Clone)]
pub enum PermissionRole {
    Admin = 0,
    Owner = 10,
    Dev = 20,
    Guest = 30,
    None = 10000,
}

impl PermissionRole {
    fn from(member_role: &MemberRole) -> Self {
        match member_role {
            MemberRole::Owner => Self::Owner,
            MemberRole::Dev => Self::Dev,
            MemberRole::Guest => Self::Guest,
        }
    }

    fn can_do_action(&self, action: ActionType) -> bool {
        (self.to_owned() as isize) <= (action as isize)
    }

    pub fn check_permission(&self, action: ActionType) -> Result<()> {
        log::debug!("check_permission {:?} do {:?}", self, action);
        if self.can_do_action(action) {
            Ok(())
        } else {
            Err(Error::Custom(405, String::from("没有权限")))
        }
    }
}

pub async fn get_user_group_role<C>(db: &C, uid: u32, group_id: u32) -> Result<PermissionRole>
where
    C: ConnectionTrait,
{
    #[derive(FromQueryResult)]
    struct GroupInfo {
        uid: u32,
        group_type: TypeVisible,
    }

    // 先查询分组信息
    let group_info = group_entity::Entity::find()
        .select_only()
        .column(group_entity::Column::Uid)
        .column_as(group_entity::Column::GroupType, "group_type")
        .filter(group_entity::Column::Id.eq(group_id))
        .into_model::<GroupInfo>()
        .one(db)
        .await?
        .ok_or_else(|| Error::Custom(404, String::from("分组不存在")))?;

    // 查询用户角色
    let user_role = user_entity::Entity::find_user_role_by_id(db, uid).await?;
    if let Some(user_role) = user_role {
        // 判断分组类型
        if group_info.group_type == TypeVisible::Private {
            // 私有分组
            if group_info.uid == uid || user_role == UserRole::Admin {
                // 本人或管理员
                Ok(PermissionRole::Dev)
            } else {
                // 非本人
                Ok(PermissionRole::None)
            }
        } else {
            // 普通分组
            if user_role == UserRole::Admin {
                // 管理员
                Ok(PermissionRole::Admin)
            } else {
                // 查询用户分组角色
                Ok(group_member_entity::Entity::find()
                    .filter(
                        group_member_entity::Column::GroupId.eq(group_id)
                            .and(group_member_entity::Column::Uid.eq(uid)),
                    )
                    .one(db)
                    .await?
                    .map_or_else(|| PermissionRole::None, |m| PermissionRole::from(&m.role)))
            }
        }
    } else {
        Ok(PermissionRole::None)
    }
}

pub async fn get_user_project_role<C>(
    db: &DatabaseConnection,
    uid: u32,
    project_id: u32,
) -> Result<PermissionRole>
where
    C: ConnectionTrait,
{
    // 查询用户角色
    let user_role = user_entity::Entity::find_user_role_by_id(db, uid).await?;
    if let Some(user_role) = user_role {
        if user_role == UserRole::Admin {
            // 管理员
            Ok(PermissionRole::Admin)
        } else {
            // 查询用户项目角色
            Ok(
                project_entity::Entity::find_project_role_by_uid(db, uid, project_id)
                    .await?
                    .map_or_else(|| PermissionRole::None, |m| PermissionRole::from(&m)),
            )
        }
    } else {
        Ok(PermissionRole::None)
    }
}
