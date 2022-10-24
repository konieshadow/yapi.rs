use anyhow::Context;
use argon2::{password_hash::SaltString, PasswordHash, Argon2};

use crate::types::UserReg;

pub async fn reg(user_reg: UserReg) {
    log::debug!("用户注册: {:?}", user_reg);
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