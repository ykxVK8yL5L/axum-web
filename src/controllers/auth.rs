use axum::{
    extract::Extension,
    Json,
    http::StatusCode,
};
use crate::{
  db::Pool,
  models::user::*,
  models::{
    response::ResponseBody,
    user_token::UserToken,
  },
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