use sea_orm::{
    ActiveEnum, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, Set,
    TransactionTrait, sea_query::Expr, ActiveValue::NotSet,
};
use yapi_common::types::{GroupAdd, GroupInfo, GroupUp, GroupWithMember, UpdateResult, DeleteResult, MemberInfo, AddMember, AddMemberResult, DeleteMember, ChangeMemberRole};
use yapi_entity::{
    base::{MemberRole, TypeVisible},
    group_entity, group_member_entity, user_entity, traits::AutoTimestamp, project_entity,
};

use crate::{error::Error, Result};

use super::base::{get_user_group_role, ActionType};

pub async fn add(
    db: &DatabaseConnection,
    group_add: GroupAdd,
    uid: u32,
) -> Result<GroupWithMember> {
    let tx = db.begin().await?;

    let mut owner_uids = group_add.owner_uids.clone();
    if !owner_uids.contains(&uid) {
        owner_uids.push(uid);
    }

    // 查询分组成员是否存在
    let member_exist_count = user_entity::Entity::find()
        .filter(user_entity::Column::Id.is_in(owner_uids.clone()))
        .count(&tx)
        .await?;

    if member_exist_count < owner_uids.clone().len() {
        return Err(Error::Custom(401, String::from("所选成员不存在")));
    }

    // 先查询组名是否存在
    let exist_count = group_entity::Entity::find()
        .filter(group_entity::Column::GroupName.eq(group_add.group_name.clone()))
        .count(&tx)
        .await?;

    if exist_count > 0 {
        return Err(Error::Custom(401, String::from("项目分组名已存在")));
    }

    // 插入分组
    let group = group_entity::ActiveModel {
        uid: Set(uid),
        group_name: Set(group_add.group_name.clone()),
        group_desc: Set(group_add.group_desc.clone()),
        group_type: Set(TypeVisible::Public),
        ..AutoTimestamp::default_add()
    };

    let group_id = group_entity::Entity::insert(group)
        .exec(&tx)
        .await?
        .last_insert_id;

    // 插入分组成员
    let group_members = owner_uids
        .iter()
        .map(|uid| group_member_entity::ActiveModel {
            group_id: Set(group_id),
            uid: Set(*uid),
            role: Set(MemberRole::Owner),
        });
    group_member_entity::Entity::insert_many(group_members)
        .exec(&tx)
        .await?;

    // 查询分组信息
    let group = group_entity::Entity::find_by_id(group_id)
        .one(&tx)
        .await?
        .ok_or_else(|| anyhow::anyhow!("insert to db failed"))?;

    // 查询成员信息
    let members = group_member_entity::Entity::find_member_by_group(&tx, group_id).await?;

    tx.commit().await?;

    Ok(GroupWithMember {
        id: group.id,
        uid: group.uid,
        group_name: group.group_name,
        group_desc: group.group_desc,
        group_type: group.group_type.into_value(),
        member: members,
    })
}

pub async fn up(db: &DatabaseConnection, group_up: GroupUp, uid: u32) -> Result<UpdateResult> {
    let tx = db.begin().await?;

    // 校验权限
    get_user_group_role(&tx, uid, group_up.id).await?
        .check_permission(ActionType::Edit)?;

    let update_model = group_entity::ActiveModel {
        group_name: group_up.group_name.map(Set).unwrap_or(NotSet),
        group_desc: group_up.group_desc.map(Set).unwrap_or(NotSet),
        ..AutoTimestamp::default_up()
    };

    let result = group_entity::Entity::update_many()
        .set(update_model)
        .filter(group_entity::Column::Id.eq(group_up.id))
        .exec(&tx)
        .await?;

    tx.commit().await?;

    Ok(result.into())
}

pub async fn del(db: &DatabaseConnection, uid: u32, group_id: u32) -> Result<DeleteResult> {
    let tx = db.begin().await?;

    // 校验权限
    get_user_group_role(&tx, uid, group_id).await?
        .check_permission(ActionType::Danger)?;

    // 删除分组
    let result = group_entity::Entity::delete_by_id(group_id)
        .exec(&tx)
        .await?;

    // 删除分组成员
    group_member_entity::Entity::delete_many()
        .filter(group_member_entity::Column::GroupId.eq(group_id))
        .exec(&tx)
        .await?;

    // 删除项目
    let project_ids = project_entity::Entity::find_project_ids_by_group(&tx, group_id).await?;
    for project_id in project_ids {
        project_entity::Entity::delete_project(&tx, project_id).await?;
    }

    tx.commit().await?;

    Ok(result.into())
}

pub async fn get_mygroup(db: &DatabaseConnection, uid: u32) -> Result<GroupInfo> {
    // 查找个人空间
    let private_group = group_entity::Entity::find_private_group(db, uid)
        .await?
        .ok_or_else(|| Error::Custom(401, String::from("分组不存在")))?;

    Ok(private_group)
}

pub async fn get(db: &DatabaseConnection, uid: u32, group_id: u32) -> Result<GroupInfo> {
    // 校验权限
    get_user_group_role(db, uid, group_id).await?
        .check_permission(ActionType::View)?;

    group_entity::Entity::find_group_info(db, uid, group_id)
        .await?
        .ok_or_else(|| Error::Custom(401, String::from("分组不存在")))
}

pub async fn list(db: &DatabaseConnection, uid: u32) -> Result<Vec<GroupInfo>> {
    // 查找个人空间
    let private_group = group_entity::Entity::find_private_group(db, uid).await?;

    // 查找加入的空间
    let mut group_list = group_entity::Entity::find_group_list(db, uid).await?;

    if let Some(group) = private_group {
        group_list.insert(0, group)
    }

    Ok(group_list)
}

pub async fn get_member_list(db: &DatabaseConnection, uid: u32, group_id: u32) -> Result<Vec<MemberInfo>> {
    // 校验权限
    get_user_group_role(db, uid, group_id).await?
        .check_permission(ActionType::View)?;
 
    group_member_entity::Entity::find_member_by_group(db, group_id).await.map_err(Into::into)
}

pub async fn add_member(db: &DatabaseConnection, uid: u32, add_member: AddMember) -> Result<AddMemberResult> {
    let member_role = MemberRole::try_from_value(&add_member.role)?;

    let tx = db.begin().await?;
    
    // 校验权限
    get_user_group_role(db, uid, add_member.id).await?
        .check_permission(ActionType::Danger)?;

    // 查询存在的用户id
    let exist_uids = user_entity::Entity::find_exist_uids(&tx, &add_member.member_uids).await?;
    log::debug!("exist_uids: {:?}", exist_uids);

    // 过滤不存在的用户id
    let no_members: Vec<u32> = add_member.member_uids.into_iter().filter(|uid| !exist_uids.contains(uid)).collect();
    log::debug!("no_members: {:?}", no_members);

    // 查询组内已存在的成员
    let exist_members = group_member_entity::Entity::find_member_by_group_and_uids(&tx, add_member.id, &exist_uids).await?;
    let exist_member_uids: Vec<u32> = exist_members.iter().map(|m| m.id).collect();
    log::debug!("exist_member_uids: {:?}", exist_member_uids);

    // 插入剩余不存在的用户id作为新成员
    let add_member_uids: Vec<u32> = exist_uids.into_iter().filter(|uid| !exist_member_uids.contains(uid)).collect();
    log::debug!("add_member_uids: {:?}", add_member_uids);
    if !add_member_uids.is_empty() {
        let models = add_member_uids.iter().map(|uid| group_member_entity::ActiveModel {
            group_id: Set(add_member.id),
            uid: Set(*uid),
            role: Set(member_role.clone())
        });
        group_member_entity::Entity::insert_many(models)
            .exec(&tx)
            .await?;
    }

    // 查询新添加的成员
    let add_members = group_member_entity::Entity::find_member_by_group_and_uids(&tx, add_member.id, &add_member_uids).await?;

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
    get_user_group_role(db, uid, delete_member.id).await?
        .check_permission(ActionType::Danger)?;

    let result = group_member_entity::Entity::delete_many()
        .filter(
            group_member_entity::Column::GroupId.eq(delete_member.id)
                .and(group_member_entity::Column::Uid.eq(delete_member.member_uid))
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
    get_user_group_role(db, uid, change_member_role.id).await?
        .check_permission(ActionType::Danger)?;

    let result = group_member_entity::Entity::update_many()
        .filter(
            group_member_entity::Column::GroupId.eq(change_member_role.id)
                .and(group_member_entity::Column::Uid.eq(change_member_role.member_uid))
        )
        .col_expr(group_member_entity::Column::Role, Expr::value(member_role))
        .exec(&tx)
        .await?;

    tx.commit().await?;

    Ok(result.into())
}