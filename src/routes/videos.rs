use axum::{
    Router,
    routing::{get,post},
};
use crate::controllers::{
    videos::{videos_home,videos_all,add,edit,del,sync,add_task},
};
use crate::{config::env::ServerConfig};

pub fn create_videos_router(config:&ServerConfig)->Router{
    let video_router = Router::new()
            .route("/videos", get(videos_home))
            .route("/videos/all", get(videos_all))
            .route("/videos/add", post(add))
            .route("/videos/edit", post(edit))
            .route("/videos/del", post(del))
            .route("/videos/addtask", post(add_task))
            .route("/videos/sync", get(sync));
    video_router
}


