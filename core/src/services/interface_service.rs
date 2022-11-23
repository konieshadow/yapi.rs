use sea_orm::{DatabaseConnection, TransactionTrait, EntityTrait, Set};
use yapi_common::types::{InterfaceAdd, InterfaceInfo};
use yapi_entity::{interface_cat_entity, interface_entity, traits::AutoTimestamp};
use crate::{Result, error::Error};

use super::base::{get_user_project_role, ActionType};

pub async fn add(db: &DatabaseConnection, uid: u32, interface_add: InterfaceAdd) -> Result<InterfaceInfo> {
    let tx = db.begin().await?;

    // 查询分类
    let interface_cat = interface_cat_entity::Entity::find_by_id(interface_add.cat_id)
        .one(&tx)
        .await?
        .ok_or_else(|| Error::Custom(401, String::from("分类不存在")))?;

    // 权限校验
    get_user_project_role(db, uid, interface_cat.project_id).await?.check_permission(ActionType::Edit)?;

    // 查询当前分类下接口最大索引
    let max_index = interface_entity::Entity::find_max_interface_index(&tx, interface_cat.project_id, interface_add.cat_id).await?;

    // 插入接口
    let interface = interface_entity::ActiveModel {
        uid: Set(uid),
        project_id: Set(interface_cat.project_id),
        cat_id: Set(interface_add.cat_id),
        index: Set(max_index + 1),
        title: Set(interface_add.title),
        method: Set(interface_add.method),
        path: Set(interface_add.path),
        ..AutoTimestamp::default_add()
    };
    let interface_id = interface_entity::Entity::insert(interface)
        .exec(&tx)
        .await?
        .last_insert_id;

    // 查询接口信息
    let interface_info = interface_entity::Entity::find_by_id(interface_id)
        .one(&tx)
        .await?
        .map(|m| m.to_interface_info())
        .ok_or_else(|| Error::Anyhow(anyhow::anyhow!("insert to db failed")))?;

    tx.commit().await?;

    Ok(interface_info)
}