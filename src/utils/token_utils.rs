use crate::{
    db::Pool,
    models::{
        user::User,
        user_token::{UserToken, KEY},
    },
};
use jwt_simple::Error;
use jwt_simple::prelude::*;


pub fn decode_token(token: String) -> Result<UserToken,Error> {
    let key = HS256Key::from_bytes(&KEY);
    //let claims = key.verify_token::<UserToken>(&token, None)?;
    let claims = match key.verify_token::<UserToken>(&token, None) {
        Ok(claims)  => claims,
        Err(e) => return Err(e),
    };
    Ok(claims.custom)

}

pub fn verify_token(token_data: &UserToken, pool: &Pool) -> Result<String, String> {
    if User::is_valid_login_session(&token_data, &pool.get().unwrap()) {
        Ok(token_data.user.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}