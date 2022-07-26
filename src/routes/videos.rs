use axum::{
    Router,
    routing::{get},
};
use crate::controllers::{
    videos::{videos_home,videos_all},
};
use crate::{config::env::ServerConfig};

pub fn create_videos_router(config:&ServerConfig)->Router{
    let video_router = Router::new()
            .route("/videos", get(videos_home))
            .route("/videos/all", get(videos_all));
    video_router
}


