use axum::{
    Router,
    routing::{get},
};
use crate::controllers::{
    index::{root,greet},
    videos::{videos_all},
};
use crate::{config::env::ServerConfig};

pub fn create_videos_router(config:&ServerConfig)->Router{
    let video_router = Router::new()
            .route("/videos/:name", get(greet))
            .route("/videos", get(root))
            .route("/videos/all", get(videos_all));
    video_router
}


