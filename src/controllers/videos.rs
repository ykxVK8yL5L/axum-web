use std::str;
use std::path::Path;
use axum::{
    body::Body,
    Json,
    extract::{Query,Extension},
    http::{Request, StatusCode},
    response::{IntoResponse},
};

use crate::{
    db::Pool,
    models::{
      response::ResponseBody,
      videos::{Video},
    },
    constants,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct VideosResult {
    pub data: Vec<Video>,
}


pub async fn videos_all(Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    match Video::get_all(&pool.get().unwrap()) {
        Ok(video) => {
            let result = VideosResult {
                data: video,
            };
            Ok(serde_json::to_string(&result).unwrap())
        }
        Err(_) =>{
            Ok(String::from("not ok"))
        } 
    }
}



