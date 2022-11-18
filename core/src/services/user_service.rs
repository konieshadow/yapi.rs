use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, ItemsAndPagesNumber,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
};
use time::OffsetDateTime;
use yapi_common::types::{PageList, Paginator, UserInfo, UserLogin, UserReg, UserSearch};
use yapi_entity::base::{TypeVisible};
use yapi_entity::{group_entity};
use yapi_entity::user_entity::{self, UserRole};

use crate::error::Error;
use crate::Result;

pub async fn reg(db: &DatabaseConnection, user_reg: UserReg) -> Result<UserInfo> {
    let tx = db.begin().await?;

    // 先查询用户名是否已存在
    let exist_count = user_entity::Entity::find()
        .filter(
            Condition::any()
                .add(user_entity::Column::Username.eq(user_reg.username.clone()))
                .add(user_entity::Column::Email.eq(user_reg.email.clone())),
        )
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
        username: Set(user_reg.username.clone()),
        email: Set(user_reg.email.clone()),
        password: Set(password),
        role: Set(UserRole::Member),
        add_time: Set(timestamp),
        up_time: Set(timestamp),
        ..Default::default()
    };

    let user_id = user_entity::Entity::insert(user)
        .exec(&tx)
        .await?
        .last_insert_id;

    let user_info = user_entity::Entity::find_user_info_by_id(&tx, user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("insert to db failed"))?;

    // 创建用户个人空间
    let user_private_gorup = group_entity::ActiveModel {
        uid: Set(user_id),
        group_name: Set(String::new()),
        group_desc: Set(String::new()),
        group_type: Set(TypeVisible::Private),
        add_time: Set(timestamp),
        up_time: Set(timestamp),
        ..Default::default()
    };
    group_entity::Entity::insert(user_private_gorup)
        .exec(&tx)
        .await?;

    tx.commit().await?;

    Ok(user_info)
}

pub async fn login(db: &DatabaseConnection, user_login: UserLogin) -> Result<UserInfo> {
    #[derive(FromQueryResult)]
    struct QueryAs {
        id: u32,
        password: String,
    }

    let user = user_entity::Entity::find()
        .select_only()
        .column(user_entity::Column::Id)
        .column(user_entity::Column::Password)
        .filter(user_entity::Column::Email.eq(user_login.email))
        .into_model::<QueryAs>()
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound(String::from("用户不存在")))?;

    let password_matched = verify_password(user_login.password, user.password).await?;

    if !password_matched {
        return Err(Error::Custom(405, String::from("密码不正确")));
    }

    let user_info = user_entity::Entity::find_user_info_by_id(db, user.id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("should be exist user"))?;

    Ok(user_info)
}

pub async fn status(db: &DatabaseConnection, user_id: Option<u32>) -> Result<UserInfo> {
    let user_id = user_id.ok_or(Error::Unauthorized)?;

    let user_info = user_entity::Entity::find_user_info_by_id(db, user_id)
        .await?
        .ok_or(Error::Unauthorized)?;

    Ok(user_info)
}

pub async fn list(db: &DatabaseConnection, paginator: Paginator) -> Result<PageList<UserInfo>> {
    let ItemsAndPagesNumber {
        number_of_items: count,
        number_of_pages: total,
    } = user_entity::Entity::find()
        .order_by_desc(user_entity::Column::Id)
        .paginate(db, paginator.page_size())
        .num_items_and_pages()
        .await?;

    let list: Vec<UserInfo> = user_entity::Entity::find()
        .order_by_desc(user_entity::Column::Id)
        .paginate(db, paginator.page_size())
        .fetch_page(paginator.page())
        .await?
        .into_iter().map(|m| m.to_user_info())
        .collect();

    Ok(PageList::new(count, total, list))
}

pub async fn search(db: &DatabaseConnection, search: &str) -> Result<Vec<UserSearch>> {
    let keyworkd = format!("%{}%", search);
    let list: Vec<UserSearch> = user_entity::Entity::find()
        .filter(
            Condition::any()
                .add(user_entity::Column::Username.like(keyworkd.as_str()))
                .add(user_entity::Column::Email.like(keyworkd.as_str()))
        )
        .order_by_desc(user_entity::Column::Id)
        .limit(50)
        .all(db)
        .await?
        .into_iter().map(|m| m.to_user_search())
        .collect();

    Ok(list)
}

async fn hash_password(password: String) -> anyhow::Result<String> {
    tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| anyhow::anyhow!("failed to geenerate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generate password hash")?
}

async fn verify_password(password: String, password_hash: String) -> anyhow::Result<bool> {
    tokio::task::spawn_blocking(move || -> anyhow::Result<bool> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        match hash.verify_password(&[&Argon2::default()], password) {
            Ok(()) => Ok(true),
            Err(e) if e == argon2::password_hash::Error::Password => Ok(false),
            Err(e) => Err(anyhow::anyhow!("falied to verify password hash: {}", e)),
        }
    })
    .await
    .context("panic in verifying password hash")?
}
