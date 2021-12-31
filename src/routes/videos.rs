use axum::{
    Router,
    routing::{get},
};
use crate::controllers::{
    index::{root,greet},
};
use crate::{config::env::ServerConfig};

pub fn create_videos_router(config:&ServerConfig)->Router{
    let video_router = Router::new()
            .route("/videos/:name", get(root))
            .route("/videos", get(root));
    video_router
}


