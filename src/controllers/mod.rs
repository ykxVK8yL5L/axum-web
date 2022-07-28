use axum::{
  extract::Extension,
  http::StatusCode,
};
use crate::{
  db::Pool,
  models,
};
automod::dir!("src/controllers");


pub async fn home(Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
  match models::user::User::find_user_by_username("hello",&pool.get().unwrap()) {
    Ok(user) => {
      let response = serde_json::to_string(&user).unwrap();
      Ok(response)
    } ,  
    Err(err) => Ok("not found user by name hello".to_string()),
  }
}
  
