mod user;

use serde::{Serialize, Deserialize};
pub use user::*;

fn page_default() -> usize {
    1
}

fn limit_default() -> usize {
    20
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginator {
    #[serde(default = "page_default")]
    page: usize,

    #[serde(default = "limit_default")]
    limit: usize,
}

impl Paginator {
    pub fn page_size(&self) -> usize {
        self.limit
    }

    pub fn page(&self) -> usize {
        if self.page == 0 {
            0
        } else {
            self.page - 1
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageList<T> {
    pub count: usize,
    pub total: usize,
    pub list: Vec<T>,
}

impl <T> PageList<T> {
    pub fn new(count: usize, total: usize, list: Vec<T>) -> Self {
        PageList { count, total, list }
    }
}