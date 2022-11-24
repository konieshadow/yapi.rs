use sea_orm::{DatabaseConnection, TransactionTrait, EntityTrait, QueryFilter, ColumnTrait, ActiveEnum};
use yapi_common::types::{InterfaceAdd, InterfaceInfo, InterfaceUp, UpdateResult, DeleteResult};
use yapi_entity::{interface_cat_entity, interface_entity::{self, InterfaceStatus, ReqHeaders, ResBodyType, ReqParams, ReqQuerys, ReqBodyType, ReqBodyForms}, traits::{AutoTimestamp}, set};
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
        uid: set!(uid),
        project_id: set!(interface_cat.project_id),
        cat_id: set!(interface_add.cat_id),
        index: set!(max_index + 1),
        title: set!(interface_add.title),
        method: set!(interface_add.method),
        path: set!(interface_add.path),
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

pub async fn up(db: &DatabaseConnection, uid: u32, interface_up: InterfaceUp) -> Result<UpdateResult> {
    let tx = db.begin().await?;

    // 查询接口基本信息
    let base_info = interface_entity::Entity::find_interface_base_info(&tx, interface_up.id)
        .await?
        .ok_or_else(|| Error::Custom(401, String::from("接口不存在")))?;

    // 权限校验
    get_user_project_role(&tx, uid, base_info.project_id).await?.check_permission(ActionType::Edit)?;

    if let Some(cat_id) = interface_up.cat_id {
        if cat_id != base_info.cat_id {
            // 检查分类是否属于该项目
            let interface_cat = interface_cat_entity::Entity::find_by_id(cat_id)
                .one(&tx)
                .await?
                .ok_or_else(|| Error::Custom(401, String::from("分类不存在")))?;
            
            if interface_cat.project_id != base_info.project_id {
                return Err(Error::Custom(401, String::from("分类不存在")));
            }
        }
    }

    let status = interface_up.status.map(|v| InterfaceStatus::try_from_value(&v)).transpose()?;
    let req_body_type = interface_up.req_body_type.map(|v| ReqBodyType::try_from_value(&v)).transpose()?;
    let res_body_type = interface_up.res_body_type.map(|v| ResBodyType::try_from_value(&v)).transpose()?;

    let update_model = interface_entity::ActiveModel {
        cat_id: set!(interface_up.cat_id),
        title: set!(interface_up.title),
        method: set!(interface_up.method),
        path: set!(interface_up.path),
        status: set!(status),
        desc: set!(interface_up.desc),
        markdown: set!(interface_up.markdown),
        req_params: set!(interface_up.req_params.map(ReqParams)),
        req_query: set!(interface_up.req_query.map(ReqQuerys)),
        req_headers: set!(interface_up.req_headers.map(ReqHeaders)),
        req_body_type: set!(req_body_type),
        req_body_is_json_schema: set!(interface_up.req_body_is_json_schema),
        req_body_form: set!(interface_up.req_body_form.map(ReqBodyForms)),
        req_body_other: set!(interface_up.req_body_other),
        res_body_type: set!(res_body_type),
        res_body: set!(interface_up.res_body),
        res_body_is_json_schema: set!(interface_up.res_body_is_json_schema),
        api_opened: set!(interface_up.api_opened),
        ..AutoTimestamp::default_up()
    };
    let result = interface_entity::Entity::update_many()
        .set(update_model)
        .filter(interface_entity::Column::Id.eq(interface_up.id))
        .exec(&tx)
        .await?;

    tx.commit().await?;

    Ok(result.into())
}

pub async fn del(db: &DatabaseConnection, uid: u32, interface_id: u32) -> Result<DeleteResult> {
    let tx = db.begin().await?;

    // 查询接口基本信息
    let base_info = interface_entity::Entity::find_interface_base_info(&tx, interface_id)
        .await?
        .ok_or_else(|| Error::Custom(401, String::from("接口不存在")))?;

    // 权限校验
    get_user_project_role(&tx, uid, base_info.project_id).await?.check_permission(ActionType::Edit)?;

    // 删除接口
    let result = interface_entity::Entity::delete_many()
        .filter(interface_entity::Column::Id.eq(interface_id))
        .exec(&tx)
        .await?;

    tx.commit().await?;

    Ok(result.into())
}

pub async fn get(db: &DatabaseConnection, uid: u32, interface_id: u32) -> Result<InterfaceInfo> {
    let interface_info = interface_entity::Entity::find_by_id(interface_id)
        .one(db)
        .await?
        .map(|m| m.to_interface_info())
        .ok_or_else(|| Error::Custom(401, String::from("接口不存在")))?;

    // 权限校验
    get_user_project_role(db, uid, interface_info.project_id).await?.check_permission(ActionType::View)?;

    Ok(interface_info)
}