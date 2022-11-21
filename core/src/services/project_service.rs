use regex::Regex;
use sea_orm::{DatabaseConnection, TransactionTrait, EntityTrait, Set, ActiveEnum, QueryFilter, ColumnTrait, ActiveValue::NotSet};
use yapi_common::types::{ProjectAdd, ProjectInfo, ProjectUp, UpdateResult};
use yapi_entity::{project_entity, base::{TypeVisible, NameValueVec}, project_env_entity, interface_cat_entity, traits::AutoTimestamp};

use crate::{Result, error::Error};

use super::base::{get_user_project_role, get_user_group_role, ActionType};

pub async fn add(db: &DatabaseConnection, uid: u32, project_add: ProjectAdd) -> Result<ProjectInfo> {
    let project_type = TypeVisible::try_from_value(&project_add.project_type)?;
    let basepath = handle_basepath(&project_add.basepath)?;

    let tx = db.begin().await?;

    // 校验权限
    get_user_group_role(db, uid, project_add.group_id).await?.check_permission(ActionType::Edit)?;

    let project = project_entity::ActiveModel {
        uid: Set(uid),
        group_id: Set(project_add.group_id),
        name: Set(project_add.name),
        basepath: Set(basepath),
        desc: Set(project_add.desc),
        project_type: Set(project_type),
        icon: Set(String::new()),
        color: Set(String::new()),
        ..AutoTimestamp::default_add()
    };

    // 插入项目
    let project_id = project_entity::Entity::insert(project)
        .exec(&tx)
        .await?
        .last_insert_id;

    // 插入默认环境
    let project_env = project_env_entity::ActiveModel {
        project_id: Set(project_id),
        name: Set(String::from("local")),
        domain: Set(String::from("http://127.0.0.1")),
        header: Set(NameValueVec::default()),
        global: Set(NameValueVec::default()),
        ..AutoTimestamp::default_add()
    };
    project_env_entity::Entity::insert(project_env)
        .exec(&tx)
        .await?;

    // 插入默认接口分类
    let interface_cat = interface_cat_entity::ActiveModel {
        uid: Set(uid),
        project_id: Set(project_id),
        name: Set(String::from("Default")),
        desc: Set(String::new()),
        ..AutoTimestamp::default_add()
    };
    interface_cat_entity::Entity::insert(interface_cat).exec(&tx).await?;

    let project_info = project_entity::Entity::find_project_info(&tx, project_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("insert to db failed"))?;

    tx.commit().await?;

    Ok(project_info)
}

pub async fn up(db: &DatabaseConnection, uid: u32, project_up: ProjectUp) -> Result<UpdateResult> {
    let project_type = project_up.project_type.map(|v| TypeVisible::try_from_value(&v)).transpose()?;
    let basepath = project_up.basepath.map(|v| handle_basepath(&v)).transpose()?;

    let tx = db.begin().await?;

    // 校验权限
    get_user_project_role(&tx, uid, project_up.id).await?.check_permission(ActionType::Edit)?;

    // 校验修改分组权限
    if let Some(group_id) = project_up.group_id {
        get_user_group_role(&tx, uid, group_id).await?.check_permission(ActionType::Edit)?;
    }

    let update_model = project_entity::ActiveModel {
        name: project_up.name.map(Set).unwrap_or(NotSet),
        group_id: project_up.group_id.map(Set).unwrap_or(NotSet),
        basepath: basepath.map(Set).unwrap_or(NotSet),
        desc: project_up.desc.map(Set).unwrap_or(NotSet),
        is_json5: project_up.is_json5.map(Set).unwrap_or(NotSet),
        switch_notice: project_up.switch_notice.map(Set).unwrap_or(NotSet),
        project_type: project_type.map(Set).unwrap_or(NotSet),
        ..AutoTimestamp::default_up()
    };

    let result = project_entity::Entity::update_many()
        .set(update_model)
        .filter(project_entity::Column::Id.eq(project_up.id))
        .exec(&tx)
        .await?;

    tx.commit().await?;

    Ok(result.into())
}

fn handle_basepath(basepath: &str) -> Result<String> {
    if basepath.is_empty() || basepath == "/" {
        return Ok(String::new());
    }

    let mut basepath = basepath.to_owned();
    if basepath[(basepath.len() - 1)..] == *"/" {
        basepath = String::from(&basepath[..(basepath.len() - 1)])
    }

    let reg = Regex::new(r"!/^/[a-zA-Z0-9\-/._]+$").unwrap();
    if reg.is_match(&basepath) {
        Err(Error::Custom(401, String::from("basepath格式有误")))
    } else {
        Ok(basepath) 
    }
}