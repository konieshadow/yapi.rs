use sea_orm::{DatabaseConnection, TransactionTrait, EntityTrait, Set, ActiveEnum};
use yapi_common::types::{ProjectAdd, ProjectInfo};
use yapi_entity::{project_entity, base::{TypeVisible, NameValueVec, AutoTimestamp}, project_env_entity, interface_cat_entity};

use crate::{Result, error::Error};

use super::base::{get_user_project_role, get_user_group_role, ActionType};

pub async fn add(db: &DatabaseConnection, uid: u32, project_add: ProjectAdd) -> Result<ProjectInfo> {
    let project_type = TypeVisible::try_from_value(&project_add.project_type)?;

    let tx = db.begin().await?;

    // 校验权限
    get_user_group_role(db, uid, project_add.group_id).await?.check_permission(ActionType::Edit)?;

    let project = project_entity::ActiveModel {
        uid: Set(uid),
        group_id: Set(project_add.group_id),
        name: Set(project_add.name),
        basepath: Set(project_add.basepath),
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