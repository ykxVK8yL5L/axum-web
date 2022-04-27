use axum::{
    Router,
    routing::{get,post},
};
use crate::controllers::{
    offline::{offline_home,offline_save_auth},
};
use crate::{config::env::ServerConfig};


pub fn create_offline_router(config:&ServerConfig)->Router{
    let web_router = Router::new().route("/offline", get(offline_home))
    .route("/offline/save_auth", post(offline_save_auth));
    web_router
}


