use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, Set,
    TransactionTrait, ActiveEnum,
};
use time::OffsetDateTime;
use yapi_common::types::{GroupAdd, GroupWithMember, GroupInfo};
use yapi_entity::{
    base::{MemberRole, TypeVisible},
    group_entity, group_member_entity, user_entity,
};

use crate::{error::Error, Result};

pub async fn add(
    db: &DatabaseConnection,
    group_add: GroupAdd,
    uid: u32,
) -> Result<GroupWithMember> {
    let tx = db.begin().await?;

    let mut owner_uids = group_add.owner_uids.to_owned();
    if !owner_uids.contains(&uid) {
        owner_uids.push(uid);
    }

    // 查询分组成员是否存在
    let member_exist_count = user_entity::Entity::find()
        .filter(user_entity::Column::Id.is_in(owner_uids.to_owned()))
        .count(&tx)
        .await?;

    if member_exist_count < owner_uids.to_owned().len() {
        return Err(Error::Custom(401, "所选成员不存在".to_owned()));
    }

    // 先查询组名是否存在
    let exist_count = group_entity::Entity::find()
        .filter(group_entity::Column::GroupName.eq(group_add.group_name.clone()))
        .count(&tx)
        .await?;

    if exist_count > 0 {
        return Err(Error::Custom(401, "项目分组名已存在".to_owned()));
    }

    let timestamp = OffsetDateTime::now_utc().unix_timestamp() as u32;

    // 插入分组
    let group = group_entity::ActiveModel {
        uid: Set(uid),
        group_name: Set(group_add.group_name.to_owned()),
        group_desc: Set(group_add.group_desc.to_owned()),
        group_type: Set(TypeVisible::Public),
        add_time: Set(timestamp),
        up_time: Set(timestamp),
        ..Default::default()
    };

    let group_id = group_entity::Entity::insert(group)
        .exec(&tx)
        .await?
        .last_insert_id;

    // 插入分组成员
    let group_member_vec = owner_uids
        .into_iter()
        .map(|uid| group_member_entity::ActiveModel {
            group_id: Set(group_id),
            uid: Set(uid),
            role: Set(MemberRole::Owner),
        });
    group_member_entity::Entity::insert_many(group_member_vec)
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

pub async fn get(db: &DatabaseConnection, uid: u32, group_id: u32) -> Result<GroupInfo> {
    group_entity::Entity::find_group_info(db, uid, group_id)
        .await?
        .ok_or_else(|| Error::Custom(401, "分组不存在".to_owned()))
}