use axum::{response::{IntoResponse, Response}, Json};
use serde::{Serialize, Deserialize};

const MSG_SUCCESS: &str = "成功！";

#[derive(Debug, Serialize, Deserialize)]
pub struct ResData<T = ()> {
    pub errcode: i32,
    pub errmsg: String,
    pub data: Option<T>,
}

impl <T> ResData<T> {
    pub fn new(errcode: i32, errmsg: String, data: T) -> Self {
        ResData { errcode, errmsg, data: Some(data) }
    }

    pub fn success(data: T) -> Self {
        ResData { errcode: 0, errmsg: MSG_SUCCESS.to_owned(), data: Some(data) }
    }

    pub fn error(errcode: i32, errmsg: String) -> Self {
        ResData { errcode, errmsg, data: None }
    }
}

impl <T> IntoResponse for ResData<T>
where T: Serialize
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}