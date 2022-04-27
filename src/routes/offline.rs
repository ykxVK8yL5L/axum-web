use axum::{
    Router,
    routing::{get},
};
use crate::controllers::{
    offline::{offline_home},
};
use crate::{config::env::ServerConfig};


pub fn create_offline_router(config:&ServerConfig)->Router{
    let web_router = Router::new().route("/offline", get(offline_home));
    web_router
}


