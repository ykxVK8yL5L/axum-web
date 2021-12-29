use crate::models::user::LoginInfoDTO;
use chrono::Utc;
use jwt_simple::prelude::*;
use serde::{ Deserialize, Serialize };

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub login_at: i64,
    pub expires_at: i64,
    pub user: String,
    pub login_session: String,
}


impl UserToken {
    pub fn generate_token(login: &LoginInfoDTO) -> String {
        let key = HS256Key::from_bytes(&KEY);
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let playload = UserToken {
            login_at: now,
            expires_at: now + ONE_WEEK,
            user: login.username.clone(),
            login_session: login.login_session.clone(),
        };
        let claims = Claims::with_custom_claims(playload, Duration::from_secs(ONE_WEEK as u64));
        let token = key.authenticate(claims).unwrap();
        token
    }
}