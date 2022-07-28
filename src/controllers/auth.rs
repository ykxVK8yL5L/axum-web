use axum::{
    body::Body,
    extract::Extension,
    Json,
    http::{Request, StatusCode},
};
use crate::{
  db::Pool,
  models::user::*,
  models::{
    response::ResponseBody,
    user_token::UserToken,
  },
  utils::token_utils,
  constants,
};


#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}



pub async fn signup(Json(user_dto): Json<UserDTO>,Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
  match User::signup(user_dto, &pool.get().unwrap()) {
      Ok(message) => {
        let response_body = ResponseBody::new(&message, constants::EMPTY);
        let response = serde_json::to_string(&response_body).unwrap();
        Ok(response)
      }
      Err(message) =>{
        let response_body = ResponseBody::new(constants::MESSAGE_SIGNUP_FAILED, message);
        let response = serde_json::to_string(&response_body).unwrap();
        Ok(response)
      } 
  }
}
 
pub async fn login(Json(login_dto): Json<LoginDTO>,Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
  match User::login(login_dto, &pool.get().unwrap()) {
      Some(logged_user) => {
        let token_response = TokenBodyResponse{ token: UserToken::generate_token(&logged_user), token_type: String::from("bearer")};
            match serde_json::to_string(&token_response) {
                Ok(token_res) => {
                    if logged_user.login_session.is_empty() {
                        let response_body = ResponseBody::new(constants::MESSAGE_CAN_NOT_FETCH_DATA, constants::MESSAGE_USER_NOT_FOUND);
                        let response = serde_json::to_string(&response_body).unwrap();
                        Ok(response)
                    } else {
                        let response_body = ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS,token_res);
                        let response = serde_json::to_string(&response_body).unwrap();
                        Ok(response)
                    }
                }
                Err(_) =>{
                        let response_body = ResponseBody::new(constants::MESSAGE_CAN_NOT_FETCH_DATA, constants::MESSAGE_INTERNAL_SERVER_ERROR);
                        let response = serde_json::to_string(&response_body).unwrap();
                        Ok(response)
                } 
            }
      }
      None =>{
        let response_body = ResponseBody::new(constants::MESSAGE_CAN_NOT_FETCH_DATA, constants::MESSAGE_USER_NOT_FOUND);
        let response = serde_json::to_string(&response_body).unwrap();
        Ok(response)
      } 
  }
}

pub async fn logout(req: Request<Body>) -> Result<String, (StatusCode, String)> {
  let pool = req.extensions().get::<Pool>().unwrap();
  if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
    if let Ok(authen_str) = authen_header.to_str() {
        if authen_str.to_lowercase().starts_with("bearer") {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(username) = token_utils::verify_token(&token_data, &pool) {
                    if let Ok(user) = User::find_user_by_username(&username, &pool.get().unwrap()) {
                        User::logout(user.id, &pool.get().unwrap());
                        let response_body = ResponseBody::new(constants::MESSAGE_LOGOUT_SUCCESS, constants::EMPTY);
                        let response = serde_json::to_string(&response_body).unwrap();
                        return Ok(response);
                    }
                }
            }
        }
    }
    let response_body = ResponseBody::new(constants::MESSAGE_LOGOUT_FAILED, constants::EMPTY);
    let response = serde_json::to_string(&response_body).unwrap();
    Ok(response)
  } else {
    let response_body =ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY);
    let response = serde_json::to_string(&response_body).unwrap();
    Ok(response)
  }

}


// pub async fn home(Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
//   match models::user::User::find_user_by_username("hello",&pool.get().unwrap()) {
//     Ok(user) => {
//       let response = serde_json::to_string(&user).unwrap();
//       Ok(response)
//     } ,  
//     Err(err) => Ok("not found user by name hello".to_string()),
//   }
// }