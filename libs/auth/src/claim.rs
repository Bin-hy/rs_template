use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claim {
    pub id: String, // 用户id
    pub exp: u64,   // 过期时间
    pub ita: u64,   // 签发时间
}

impl Display for Claim {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "id:{}\n expire:{}", self.id, self.exp)
    }
}
