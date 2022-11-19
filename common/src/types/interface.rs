use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceCat {
    pub id: u32,
    pub uid: u32,
    pub name: String,
    pub project_id: u32,
    pub desc: String,
    pub add_time: u32,
    pub up_time: u32,
}