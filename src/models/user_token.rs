use crate::models::user::LoginInfoDTO;
use chrono::Utc;
use serde::{ Deserialize, Serialize };

use hmac::{Hmac, NewMac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;


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
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let playload = UserToken {
            login_at: now,
            expires_at: now + ONE_WEEK,
            user: login.username.clone(),
            login_session: login.login_session.clone(),
        };
        let header: Header = Default::default();
        let unsigned_token = Token::new(header, playload);
        let key: Hmac<Sha256> = Hmac::new_from_slice(&KEY).map_err(|_e| "Invalid key").unwrap();
        let signed_token = unsigned_token.sign_with_key(&key).map_err(|_e| "Sign error").unwrap();
        signed_token.into()

    }
}