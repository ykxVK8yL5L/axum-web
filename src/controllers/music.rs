use std::str;
use std::path::Path;
use download_rs::async_download::Download;
use crate::utils::{template::{HtmlTemplate,MusicTemplate}};
use axum::{
    Json,
    extract::{Query,Extension},
    http::{StatusCode},
    response::{IntoResponse},
};

use crate::{
    models::{
      response::ResponseBody,
    },
    constants,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicQueryRequest {
    pub data: String,
    pub v: String,
}

#[derive(Deserialize)]
pub struct MusicLrcQueryRequest {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct SongRequest {
    pub name: String,
    pub artist: String,
    pub cover: String,
    pub url_m4a: Option<String>,
    pub url_320: Option<String>,
    pub url_flac: Option<String>,
    pub url: String,
    pub lrc: String,
}

pub async fn music_home() -> impl IntoResponse {
    let name = "音乐下载".to_string();
    let music_template = MusicTemplate { name };
    HtmlTemplate(music_template)
}
pub async fn music_query(Json(mrq): Json<MusicQueryRequest>) -> Result<String, (StatusCode, String)> {
    let bytes = vec![104u8, 116, 116, 112, 58, 47, 47, 53, 57, 46, 49, 49, 48, 46, 52, 53, 46, 50, 56, 47, 109, 47, 97, 112, 105, 47, 115, 101, 97, 114, 99, 104];
    let url = match str::from_utf8(&bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let response = reqwest::Client::new()
        .post(url)
        .json(&mrq)
        .send()
        .await
        .expect("send");
    let msg = response.text().await.expect("some content");
    Ok(msg)      
}
pub async fn music_lrc_query(mlqr: Query<MusicLrcQueryRequest>) -> Result<String, (StatusCode, String)> {
    let lrc_query: MusicLrcQueryRequest = mlqr.0;
    let query_url = lrc_query.url;
    let body = reqwest::get(query_url)
        .await
        .expect("send")
        .text()
        .await
        .expect("send");
    Ok(body)      
}
pub async fn music_download(Json(song): Json<SongRequest>,Extension(save_dir): Extension<String>) -> Result<String, (StatusCode, String)> {
    let filename = format!("{}-{}",song.name,song.artist);
    let mp3_path_name = format!("{}/mp3/{}.mp3",save_dir,filename.clone());
    let cover_path_name = format!("{}/cover/{}.png",save_dir,filename.clone());
    let lrc_path_name = format!("{}/lrc/{}.lrc",save_dir,filename.clone());

    // let songDto = SongDTO{
    //     name: song.name,
    //     artist: song.artist,
    //     filename: Some(filename.clone()),
    // };

    match Path::new(&mp3_path_name).exists() {
        true => { 
            let response_body = ResponseBody::new(constants::MESSAGE_FAIL,"文件已经存在，如果一定要下载请删除后再下载".to_string());
            let response = serde_json::to_string(&response_body).unwrap();
            Ok(response) 
        }
        false => { 
            println!("开始下载{}",filename.clone());

            match song.url_320 {
                Some(url) => {
                    let mp3_download = Download::new(&url,Some(&mp3_path_name),None);
                    match mp3_download.download() {
                        Ok(_) => println!("mp3下载完成"),
                        Err(e) => println!("mp3下载出错 ： {}",e.to_string()),
                    }
                },
                None => {
                    let mp3_download = Download::new(&song.url,Some(&mp3_path_name),None);
                    match mp3_download.download() {
                        Ok(_) => println!("mp3下载完成"),
                        Err(e) => println!("mp3下载出错: {}",e.to_string()),
                    }
                },
            };


            let cover_download = Download::new(&song.cover,Some(&cover_path_name),None);
            match cover_download.download() {
                Ok(_) => println!("cover下载完成"),
                Err(e) => println!("cover下载出错 : {}",e.to_string()),
            }
            let lrc_download = Download::new(&song.lrc,Some(&lrc_path_name),None);
            match lrc_download.download() {
                Ok(_) => println!("lrc下载完成"),
                Err(e) => println!("lrc下载出错 ： {}",e.to_string()),
            }

            let response_body = ResponseBody::new(constants::MESSAGE_OK,"下载完成".to_string());
            let response = serde_json::to_string(&response_body).unwrap();
            Ok(response)
        } 
    }

}








