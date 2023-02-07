use std::fs;
use tower_http::{services::ServeDir,add_extension::AddExtensionLayer, trace::TraceLayer};
use axum::{Router,routing::{get,post,get_service},http::StatusCode,};
use crate::controllers::music;
use crate::{config::env::ServerConfig};


pub fn create_music_router(config:&ServerConfig)->Router{
    let dir = config.root.to_string();
    fs::create_dir_all(format!("{}/music-data/mp3",dir));
    fs::create_dir_all(format!("{}/music-data/cover",dir));
    fs::create_dir_all(format!("{}/music-data/lrc",dir));

    let music_data_dir = format!("{}/music-data",dir);
    let music_routes = Router::new()
    .route("/music", get(music::music_home))
    .route("/musicquery",post(music::music_query))
    .route("/musiclrcquery",get(music::music_lrc_query))
    .route("/musicdownload",post(music::music_download))
    // .nest("/music-data",get_service(ServeDir::new(music_data_dir.clone())).handle_error(|error: std::io::Error| async move {
    //         (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             format!("Unhandled internal error: {}", error),
    //         )
    //     }),
    // )
    .layer(AddExtensionLayer::new(music_data_dir));
    music_routes
}


