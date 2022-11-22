use sea_orm::{DatabaseConnection, TransactionTrait, EntityTrait, Set, QueryFilter, ColumnTrait, ActiveValue::NotSet};
use yapi_common::types::{AddInterfaceCat, InterfaceCat, UpInterfaceCat, UpdateResult, DeleteResult};
use yapi_entity::{interface_cat_entity, traits::AutoTimestamp, interface_entity};
use crate::{Result, error::Error};

use super::base::{get_user_project_role, ActionType};

pub async fn add_interface_cat(db: &DatabaseConnection, uid: u32, add_interface_cat: AddInterfaceCat) -> Result<InterfaceCat> {
    let tx = db.begin().await?;

    // 权限校验
    get_user_project_role(&tx, uid, add_interface_cat.project_id).await?.check_permission(ActionType::Edit)?;

    // 查询当前分类最大索引
    let max_cat_index = interface_cat_entity::Entity::find_max_interface_cat_index(db, add_interface_cat.project_id).await?;

    let interface_cat = interface_cat_entity::ActiveModel {
        uid: Set(uid),
        index: Set(max_cat_index + 1),
        name: Set(add_interface_cat.name),
        project_id: Set(add_interface_cat.project_id),
        desc: Set(add_interface_cat.desc),
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

pub async fn up_interface_cat(db: &DatabaseConnection, uid: u32, up_interface_cat: UpInterfaceCat) -> Result<UpdateResult> {
    let tx = db.begin().await?;

    // 查询分类信息
    let interface_cat = interface_cat_entity::Entity::find_by_id(up_interface_cat.id)
        .one(&tx)
        .await?
        .ok_or_else(|| Error::Custom(401, String::from("分类不存在")))?;

    // 权限校验
    get_user_project_role(&tx, uid, interface_cat.project_id).await?.check_permission(ActionType::Edit)?;

    let update_model = interface_cat_entity::ActiveModel {
        name: up_interface_cat.name.map(Set).unwrap_or(NotSet),
        desc: up_interface_cat.desc.map(Set).unwrap_or(NotSet),
        ..AutoTimestamp::default_up()
    };
    let result = interface_cat_entity::Entity::update_many()
        .filter(interface_cat_entity::Column::Id.eq(up_interface_cat.id))
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
        .filter(interface_entity::Column::CatId.eq(cat_id))
        .exec(&tx)
        .await?;

    tx.commit().await?;

    Ok(result.into())
}