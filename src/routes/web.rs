use axum::{
    Router,
    routing::{get},
    response::{IntoResponse},
};
use crate::controllers::{
    index::{root,greet},
    offline::{offline_home},
};
use crate::{config::env::ServerConfig};
use std::fs;
use crate::utils::{template::{HtmlTemplate,HomeTemplate}};


pub async fn home() -> impl IntoResponse {
    let name = "首页".to_string();
    //let auth_info = fs::read_to_string("setting.json").expect("Something went wrong reading the file");
    let auth_info = match fs::read_to_string("setting.json") {
        Err(e) => "{}".to_string(),
        Ok(f) => f,
    };
    let home_template = HomeTemplate { name, auth_info };
    HtmlTemplate(home_template)
}


pub fn create_web_router(config:&ServerConfig)->Router{
    let web_router = Router::new().route("/greet/:name", get(greet))
    .route("/", get(home))
    .route("/hello", get(root));
    web_router
}


