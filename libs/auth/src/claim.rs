use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub id: String, // 用户id
    pub exp: u64,   // 过期时间
    pub ita: u64,   // 签发时间
}

impl Display for Claim {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "id:{}\n expire:{}\n ita:{}", self.id, self.exp, self.ita)
    }
}

#[test]
fn test_claim() {
    use chrono::{Utc, TimeZone, Duration};
    use crate::claim::Claim;
    let ita_now = Utc::now().timestamp();
    let offset_ts = (Utc::now() + Duration::hours(1)).timestamp();
    let claim  = Claim{
        id: "1".to_string(),
        exp: offset_ts as u64,
        ita: ita_now as u64,
    };
    println!("{}", claim);
}
