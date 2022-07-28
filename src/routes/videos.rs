use axum::{
    Router,
    routing::{get,post},
};
use tower_http::add_extension::AddExtensionLayer;
use crate::controllers::{
    videos::{videos_home,videos_all,add,edit,del,sync,add_task,create_m3u},
};
use crate::{config::env::ServerConfig};

pub fn create_videos_router(config:&ServerConfig)->Router{
    let dir = config.root.to_string();
    let webdav_root = format!("{}/music-data",&dir.to_string());

    let video_router = Router::new()
            .route("/videos", get(videos_home))
            .route("/videos/all", get(videos_all))
            .route("/videos/add", post(add))
            .route("/videos/edit", post(edit))
            .route("/videos/del", post(del))
            .route("/videos/addtask", post(add_task))
            .route("/videos/sync", get(sync))
            .route("/videos/createm3u", get(create_m3u))
            .layer(AddExtensionLayer::new(webdav_root));
    video_router
}


