use axum::{
    Router,
    routing::{get},
};
use crate::controllers::{
    index::{root,greet},
    offline::{offline_home},
};
use crate::{config::env::ServerConfig};


pub fn create_web_router(config:&ServerConfig)->Router{
    let web_router = Router::new().route("/greet/:name", get(greet))
    .route("/hello", get(root));
    web_router
}


