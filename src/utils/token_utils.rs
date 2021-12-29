use crate::{
    db::Pool,
    models::{
        user::User,
        user_token::{UserToken, KEY},
    },
};

use hmac::{Hmac, NewMac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

pub fn decode_token(token: String) -> Result<UserToken,&'static str> {
   
    let key: Hmac<Sha256> = Hmac::new_from_slice(&KEY).map_err(|_e| "Invalid key").unwrap();
    let token: Token<Header, UserToken, _> = match token.verify_with_key(&key).map_err(|_e| "Verification failed") {
        Ok(claims)  => claims,
        Err(e) => return Err(e),
    };

    let (_, claims) = token.into();
    Ok(claims)



}

pub fn verify_token(token_data: &UserToken, pool: &Pool) -> Result<String, String> {
    if User::is_valid_login_session(&token_data, &pool.get().unwrap()) {
        Ok(token_data.user.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}