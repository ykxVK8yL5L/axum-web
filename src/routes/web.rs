use axum::{
    Router,
    routing::{get,post},
    response::{IntoResponse},
};
use crate::controllers::{
    index::{root,greet,password_generator},
    gateways,
    settings,
};
use crate::{config::env::ServerConfig};
// use std::fs;
use crate::utils::{template::{HtmlTemplate,HomeTemplate,PasswordGeneratorTemplate}};


pub async fn home() -> impl IntoResponse {
    let name = "首页".to_string();
    //let auth_info = fs::read_to_string("setting.json").expect("Something went wrong reading the file");
    // let auth_info = match fs::read_to_string("setting.json") {
    //     Err(e) => "{}".to_string(),
    //     Ok(f) => f,
    // };
    let home_template = HomeTemplate { name };
    HtmlTemplate(home_template)
}


pub fn create_web_router(config:&ServerConfig)->Router{
    let web_router = Router::new().route("/greet/:name", get(greet))
    .route("/", get(home))
    .route("/hello", get(root))
    .route("/password", get(password_generator))
    .route("/gateways/all", get(gateways::all))
    .route("/gateways/add", post(gateways::add))
    .route("/gateways/save", post(gateways::save))
    .route("/gateways/del", post(gateways::del))
    .route("/settings/all", get(settings::all))
    .route("/settings/save", post(settings::save));
    web_router
}


