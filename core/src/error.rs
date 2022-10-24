use std::{borrow::Cow, collections::HashMap};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::res::ResData;

const MSG_BAD_REQUEST: &str = "请求参数格式不正确";
const MSG_UNAUTHORIZED: &str = "未登录";
const MSG_FORBIDDEN: &str = "没有权限";
const MSG_NOT_FOUND: &str = "请求资源未找到";
const MSG_UNPROCESSABLE_ENTITY: &str = "请求参数格式不正确";
const MSG_ANYHOW: &str = "服务器异常";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error with custum code and message")]
    Custom { code: i32, msg: String },

    #[error("bad request")]
    BadRequest,

    #[error("authentication required")]
    Unauthorized,

    #[error("user may not perform that action")]
    Forbidden,

    #[error("request resource not found")]
    NotFound { msg: String },

    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    #[error("error when validate request payload")]
    ValidatorError(#[from] validator::ValidationErrors),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    fn errcode(&self) -> i32 {
        match self {
            Self::Custom { code, .. } => *code,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound { .. } => 404,
            Self::UnprocessableEntity { .. } => 422,
            Self::ValidatorError(_) => 422,
            Self::Anyhow(_) => 500,
        }
    }

    fn errmsg(&self) -> String {
        match self {
            Self::Custom { msg, .. } => msg.clone(),
            Self::BadRequest => String::from(MSG_BAD_REQUEST),
            Self::Unauthorized => String::from(MSG_UNAUTHORIZED),
            Self::Forbidden => String::from(MSG_FORBIDDEN),
            Self::NotFound { msg } => {
                if msg.is_empty() {
                    String::from(MSG_NOT_FOUND)
                } else {
                    msg.clone()
                }
            }
            Self::UnprocessableEntity { errors } => {
                if errors.is_empty() {
                    String::from(MSG_UNPROCESSABLE_ENTITY)
                } else {
                    format!(
                        "{} {}",
                        MSG_UNPROCESSABLE_ENTITY,
                        serde_json::to_string(errors).expect("panic when serialize to json")
                    )
                }
            }
            Self::ValidatorError(err) => {
                format!(
                    "{} {}",
                    MSG_UNPROCESSABLE_ENTITY,
                    serde_json::to_string(err).expect("panic when serialize to json")
                )
            }
            Self::Anyhow(_) => String::from(MSG_ANYHOW),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let res_data: ResData<()> = ResData::error(self.errcode(), self.errmsg());
        (StatusCode::OK, Json(res_data)).into_response()
    }
}
