use regex::Regex;
use sea_orm::{DatabaseConnection, TransactionTrait, EntityTrait, Set, ActiveEnum, QueryFilter, ColumnTrait, ActiveValue::NotSet, sea_query::Expr};
use yapi_common::types::{ProjectAdd, ProjectInfo, ProjectUp, UpdateResult, DeleteResult, ProjectDetail, ProjectList, List, ProjectItem, MemberInfo, DeleteMember, AddMember, AddMemberResult, ChangeMemberRole};
use yapi_entity::{project_entity, base::{TypeVisible, NameValueVec, MemberRole}, project_env_entity, interface_cat_entity, traits::AutoTimestamp, project_member_entity, user_entity};

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

pub async fn del(db: &DatabaseConnection, uid: u32, project_id: u32) -> Result<DeleteResult> {
    let tx = db.begin().await?;

    // 权限校验
    get_user_project_role(&tx, uid, project_id).await?.check_permission(ActionType::Danger)?;

    // 删除项目
    let result = project_entity::Entity::delete_many()
        .filter(project_entity::Column::Id.eq(project_id))
        .exec(&tx)
        .await?;

    // 删除项目环境
    project_env_entity::Entity::delete_many()
        .filter(project_env_entity::Column::ProjectId.eq(project_id))
        .exec(&tx)
        .await?;

    // 删除项目成员
    project_member_entity::Entity::delete_many()
        .filter(project_member_entity::Column::ProjectId.eq(project_id))
        .exec(&tx)
        .await?;

    // 删除项目下的接口分类
    // TODO

    // 删除项目下的所有接口
    // TODO

    tx.commit().await?;

    Ok(result.into())
}

pub async fn get(db: &DatabaseConnection, uid: u32, project_id: u32) -> Result<ProjectDetail> {
    // 权限校验
    get_user_project_role(db, uid, project_id).await?.check_permission(ActionType::View)?;

    // 查询项目基本信息
    let project_info = project_entity::Entity::find_project_info(db, project_id).await?
        .ok_or_else(|| Error::Custom(401, String::from("项目不存在")))?;

    // 查询项目接口分类
    let cat = interface_cat_entity::Entity::find_interface_cat_by_project(db, project_id).await?;

    Ok(ProjectDetail { project_info, cat })
}

pub async fn list(db: &DatabaseConnection, uid: u32, query: ProjectList) -> Result<List<ProjectItem>> {
    // 权限校验
    get_user_group_role(db, uid, query.group_id).await?.check_permission(ActionType::View)?;

    let list = project_entity::Entity::find_project_list_by_group(db, query).await?;

    Ok(List::new(list))
}

pub async fn get_member_list(db: &DatabaseConnection, uid: u32, project_id: u32) -> Result<Vec<MemberInfo>> {
    // 权限校验
    get_user_project_role(db, uid, project_id).await?.check_permission(ActionType::Danger)?;

    let list = project_member_entity::Entity::find_member_by_project(db, project_id).await?;
    
    Ok(list)
}

pub async fn add_member(db: &DatabaseConnection, uid: u32, add_member: AddMember) -> Result<AddMemberResult> {
    let member_role = MemberRole::try_from_value(&add_member.role)?;

    let tx = db.begin().await?;
    
    // 校验权限
    get_user_project_role(db, uid, add_member.id).await?
        .check_permission(ActionType::Danger)?;

    // 查询存在的用户id
    let exist_uids = user_entity::Entity::find_exist_uids(&tx, &add_member.member_uids).await?;
    log::debug!("exist_uids: {:?}", exist_uids);

    // 过滤不存在的用户id
    let no_members: Vec<u32> = add_member.member_uids.clone().into_iter().filter(|uid| !exist_uids.contains(uid)).collect();
    log::debug!("no_members: {:?}", no_members);

    // 查询组内已存在的成员
    let exist_members = project_member_entity::Entity::find_member_by_project_and_uids(&tx, add_member.id, &exist_uids).await?;
    let exist_member_uids: Vec<u32> = exist_members.clone().into_iter().map(|m| m.id).collect();
    log::debug!("exist_member_uids: {:?}", exist_member_uids);

    // 插入剩余不存在的用户id作为新成员
    let add_member_uids: Vec<u32> = exist_uids.into_iter().filter(|uid| !exist_member_uids.contains(uid)).collect();
    log::debug!("add_member_uids: {:?}", add_member_uids);
    if !add_member_uids.is_empty() {
        let models = add_member_uids.clone().into_iter().map(|uid| project_member_entity::ActiveModel {
            project_id: Set(add_member.id),
            uid: Set(uid),
            role: Set(member_role.clone())
        });
        project_member_entity::Entity::insert_many(models)
            .exec(&tx)
            .await?;
    }

    // 查询新添加的成员
    let add_members = project_member_entity::Entity::find_member_by_project_and_uids(&tx, add_member.id, &add_member_uids).await?;

    tx.commit().await?;

    Ok(AddMemberResult {
        add_members,
        exist_members,
        no_members
    })
}

pub async fn delete_member(db: &DatabaseConnection, uid: u32, delete_member: DeleteMember) -> Result<DeleteResult> {
    let tx = db.begin().await?;

    // 校验权限
    get_user_project_role(db, uid, delete_member.id).await?
        .check_permission(ActionType::Danger)?;

    let result = project_member_entity::Entity::delete_many()
        .filter(
            project_member_entity::Column::ProjectId.eq(delete_member.id)
                .and(project_member_entity::Column::Uid.eq(delete_member.member_uid))
        )
        .exec(&tx)
        .await?;

    tx.commit().await?;

    Ok(result.into())
}

pub async fn change_member_role(db: &DatabaseConnection, uid: u32, change_member_role: ChangeMemberRole) -> Result<UpdateResult> {
    let member_role = MemberRole::try_from_value(&change_member_role.role)?;

    let tx = db.begin().await?;

    // 校验权限
    get_user_project_role(db, uid, change_member_role.id).await?
        .check_permission(ActionType::Danger)?;

    let result = project_member_entity::Entity::update_many()
        .filter(
            project_member_entity::Column::ProjectId.eq(change_member_role.id)
                .and(project_member_entity::Column::Uid.eq(change_member_role.member_uid))
        )
        .col_expr(project_member_entity::Column::Role, Expr::value(member_role))
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