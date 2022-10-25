use anyhow::Context;
use argon2::{password_hash::SaltString, PasswordHash, Argon2};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, PaginatorTrait, ColumnTrait, Set, Condition, TransactionTrait};
use time::OffsetDateTime;
use yapi_common::types::{UserReg, UserInfo};
use yapi_entity::user_entity::{self, UserRole};

use crate::error::Error;
use crate::Result;

pub async fn reg(db: &DatabaseConnection, user_reg: UserReg) -> Result<UserInfo> {
    log::debug!("用户注册: {:?}", user_reg);

    let tx = db.begin().await?;

    // 先查询用户名是否已存在
    let exist_count = user_entity::Entity::find()
        .filter(
            Condition::any()
                .add(user_entity::Column::Username.eq(user_reg.username.to_owned()))
                .add(user_entity::Column::Email.eq(user_reg.email.to_owned())
        ))
        .count(&tx)
        .await?;

    if exist_count > 0 {
        return Err(Error::Custom(401, String::from("该用户名或邮箱已存在")));
    }

    // 加密密码
    let password = hash_password(user_reg.password).await?;

    let timestamp = OffsetDateTime::now_utc().unix_timestamp() as u32;

    // 插入记录
    let user = user_entity::ActiveModel {
        username: Set(user_reg.username.to_owned()),
        email: Set(user_reg.email.to_owned()),
        password: Set(password),
        role: Set(UserRole::Member),
        add_time: Set(timestamp),
        up_time: Set(timestamp),
        ..Default::default()
    };

    let user_id = user_entity::Entity::insert(user).exec(&tx).await?.last_insert_id;

    let user_info = user_entity::Entity::find_user_info_by_id(&tx, user_id).await?.expect("must insert successful");

    tx.commit().await?;

    Ok(user_info)
}

async fn hash_password(password: String) -> anyhow::Result<String> {
    tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(PasswordHash::generate(Argon2::default(), password, salt.as_str())
            .map_err(|e| anyhow::anyhow!("failed to geenerate password hash: {}", e))?
            .to_string())
    })
    .await
    .context("panic in generate password hash")?
}

async fn verify_password(password: String, password_hash: String) -> anyhow::Result<bool> {
    tokio::task::spawn_blocking(move || -> anyhow::Result<bool> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        match hash.verify_password(&[&Argon2::default()], password) {
            Ok(()) => {
                Ok(true)
            },
            Err(e) if e == argon2::password_hash::Error::Password => {
                Ok(false)
            },
            Err(e) => {
                Err(anyhow::anyhow!("falied to verify password hash: {}", e))
            }
        }
    })
    .await
    .context("panic in verifying password hash")?
}