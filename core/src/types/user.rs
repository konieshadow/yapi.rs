use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref RE_PASSWORD: Regex = Regex::new(r"[A-Za-z\d$@$!%*#?&]{8,20}").unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserReg {
    #[validate(length(min = 2, max = 50, message = "长度必须在2到50之间"))]
    pub username: String,

    #[validate(email(message = "必须是合法的邮箱地址"))]
    pub email: String,

    #[validate(regex(path ="RE_PASSWORD", message = "必须是8到20位的数字、字母与英文符号的组合"))]
    pub password: String,
}
