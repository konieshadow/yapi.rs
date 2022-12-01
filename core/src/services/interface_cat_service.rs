use sea_orm::{DatabaseConnection, TransactionTrait, EntityTrait, Set, QueryFilter, ColumnTrait, ActiveValue::NotSet, PaginatorTrait, sea_query::Expr};
use yapi_common::types::{InterfaceCatAdd, InterfaceCat, InterfaceCatUp, UpdateResult, DeleteResult, IndexItem};
use yapi_entity::{interface_cat_entity, traits::AutoTimestamp, interface_entity};
use crate::{Result, error::Error};

use super::base::{get_user_project_role, ActionType};

pub async fn add_interface_cat(db: &DatabaseConnection, uid: u32, interface_cat_add: InterfaceCatAdd) -> Result<InterfaceCat> {
    let tx = db.begin().await?;

    // 权限校验
    get_user_project_role(&tx, uid, interface_cat_add.project_id).await?.check_permission(ActionType::Edit)?;

    // 查询当前分类最大索引
    let max_cat_index = interface_cat_entity::Entity::find_max_interface_cat_index(db, interface_cat_add.project_id).await?;

    let interface_cat = interface_cat_entity::ActiveModel {
        uid: Set(uid),
        index: Set(max_cat_index + 1),
        name: Set(interface_cat_add.name),
        project_id: Set(interface_cat_add.project_id),
        desc: Set(interface_cat_add.desc),
        ..AutoTimestamp::default_add()
    };
    let interface_cat_id = interface_cat_entity::Entity::insert(interface_cat)
        .exec(&tx)
        .await?
        .last_insert_id;

    let result = interface_cat_entity::Entity::find_by_id(interface_cat_id)
        .one(&tx)
        .await?
        .map(|m| m.to_interface_cat())
        .ok_or_else(|| anyhow::anyhow!("insert to db failed"))?;

    tx.commit().await?;

    Ok(result)
}

pub async fn up_interface_cat(db: &DatabaseConnection, uid: u32, interface_cat_up: InterfaceCatUp) -> Result<UpdateResult> {
    let tx = db.begin().await?;

    // 查询分类信息
    let interface_cat = interface_cat_entity::Entity::find_by_id(interface_cat_up.id)
        .one(&tx)
        .await?
        .ok_or_else(|| Error::Custom(401, String::from("分类不存在")))?;

    // 权限校验
    get_user_project_role(&tx, uid, interface_cat.project_id).await?.check_permission(ActionType::Edit)?;

    let update_model = interface_cat_entity::ActiveModel {
        name: interface_cat_up.name.map(Set).unwrap_or(NotSet),
        desc: interface_cat_up.desc.map(Set).unwrap_or(NotSet),
        ..AutoTimestamp::default_up()
    };
    let result = interface_cat_entity::Entity::update_many()
        .filter(interface_cat_entity::Column::Id.eq(interface_cat_up.id))
        .set(update_model)
        .exec(&tx)
        .await?;

    tx.commit().await?;

    Ok(result.into())
}

pub async fn delete_interface_cat(db: &DatabaseConnection, uid: u32, cat_id: u32) -> Result<DeleteResult> {
    let tx = db.begin().await?;

    // 查询分类信息
    let interface_cat = interface_cat_entity::Entity::find_by_id(cat_id)
        .one(&tx)
        .await?
        .ok_or_else(|| Error::Custom(401, String::from("分类不存在")))?;

    // 权限校验
    get_user_project_role(&tx, uid, interface_cat.project_id).await?.check_permission(ActionType::Edit)?;

    // 修改该分类后面的索引值
    interface_cat_entity::Entity::update_interface_cat_index_after(&tx, interface_cat.project_id, interface_cat.index).await?;

    // 删除分类
    let result = interface_cat_entity::Entity::delete_many()
        .filter(interface_cat_entity::Column::Id.eq(cat_id))
        .exec(&tx)
        .await?;

    // 删除分类下的接口
    interface_entity::Entity::delete_many()
        .filter(
            interface_entity::Column::ProjectId.eq(interface_cat.project_id)
                .and(interface_entity::Column::CatId.eq(cat_id))
        )
        .exec(&tx)
        .await?;

    tx.commit().await?;

    Ok(result.into())
}

pub async fn up_interface_cat_index(db: &DatabaseConnection, uid: u32, index_list: Vec<IndexItem>) -> Result<UpdateResult> {
    let tx = db.begin().await?;

    // 校验所传分类是否在同一个项目里
    let mut cat_ids: Vec<u32> = index_list.iter().map(|item| item.id).collect();
    // 去重
    cat_ids.sort_unstable();
    cat_ids.dedup();

    let project_ids = interface_cat_entity::Entity::find_project_ids_by_interface_cat_ids(&tx, &cat_ids).await?;
    if project_ids.len() != 1 {
        return Err(Error::Custom(402, String::from("分类列表不完整")));
    }

    let project_id = project_ids[0];
    // 权限校验
    get_user_project_role(&tx, uid, project_id).await?.check_permission(ActionType::Edit)?;

    // 查询该项目分类数量
    let cat_count = interface_cat_entity::Entity::find()
        .filter(interface_cat_entity::Column::ProjectId.eq(project_id))
        .count(&tx)
        .await?;

    if cat_count != cat_ids.len() {
        return Err(Error::Custom(402, String::from("分类列表不完整")));
    }

    // 循环更新索引
    for item in index_list {
        interface_cat_entity::Entity::update_many()
            .col_expr(interface_cat_entity::Column::Index, Expr::value(item.index))
            .filter(interface_cat_entity::Column::Id.eq(item.id))
            .exec(&tx)
            .await?;
    }

    tx.commit().await?;

    Ok(UpdateResult::of(cat_count as u32))
}