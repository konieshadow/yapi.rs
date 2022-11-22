use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceCat {
    pub id: u32,
    pub uid: u32,
    pub index: u32,
    pub name: String,
    pub project_id: u32,
    pub desc: String,
    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AddInterfaceCat {
    #[validate(length(min = 1, max = 30, message = "长度必须在1到30之间"))]
    pub name: String,

    pub project_id: u32,

    #[validate(length(max = 200, message = "长度不能大于200"))]
    pub desc: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpInterfaceCat {
    pub id: u32,

    #[validate(length(min = 1, max = 30, message = "长度必须在1到30之间"))]
    pub name: Option<String>,

    #[validate(length(max = 200, message = "长度不能大于200"))]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexItem {
    pub id: u32,
    pub index: u32,
}