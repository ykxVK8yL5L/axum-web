use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use axum::{
    Json,
    extract::{Query,Extension},
    http::{StatusCode},
    response::{IntoResponse},
};
use tracing::{info, debug};
use crate::utils::template::{HtmlTemplate,VideoTemplate};
use crate::{
    db::Pool,
    models::{
      videos::{Video,VideoDTO},
      settings::{Setting},
    },
};


pub async fn videos_home(Extension(pool): Extension<Pool>,) -> impl IntoResponse {
    let name = "IPFS文件管理".to_string();
    let gateway = match Setting::find_value_by_key(&"gateway".to_string(), &pool.get().unwrap()) {
        Ok(gateway) => gateway,
        Err(err) => {
            debug!("{:?}", err);
            "".to_string()
        }
    };
    let hellotemplate =VideoTemplate { name,gateway };
    HtmlTemplate(hellotemplate)
}


pub async fn videos_all(Extension(pool): Extension<Pool>,Query(params): Query<HashMap<String, String>>,) -> Result<String, (StatusCode, String)> {
    //debug!("{}", params.get("search[value]").unwrap());
    match Video::pagination(&params,&pool.get().unwrap()) {
        Ok(result) => {
            Ok(result)
        }
        Err(_) =>{
            Ok(String::from("not ok"))
        } 
    }
}

pub async fn add(Extension(pool): Extension<Pool>,Json(video): Json<VideoDTO>,) -> Result<String, (StatusCode, String)> {
    match Video::insert(video,&pool.get().unwrap()) {
        Ok(_) => {
            Ok(String::from("添加成功"))
        }
        Err(_) =>{
            Ok(String::from("添加失败请稍后再试"))
        } 
    }
}


pub async fn edit(Extension(pool): Extension<Pool>,Query(params): Query<HashMap<String, String>>,Json(video): Json<VideoDTO>,) -> Result<String, (StatusCode, String)> {
    let video_id = match params.get("id") {
        Some(id) => id.parse::<i32>().unwrap(),
        None => {
            0
        }
    }; 

    if video_id == 0 {
        return Ok(String::from("ID不能为空"))
    }
    match Video::edit(video_id,video,&pool.get().unwrap()) {
        Ok(_) => {
            Ok(String::from("修改成功"))
        }
        Err(_) =>{
            Ok(String::from("修改失败请稍后再试"))
        } 
    }
}




pub async fn del(Query(params): Query<HashMap<String, String>>,Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    let video_id = match params.get("id") {
        Some(id) => id.parse::<i32>().unwrap(),
        None => {
            0
        }
    }; 

    if video_id == 0 {
        return Ok(String::from("ID不能为空"))
    }

    match Video::delete(video_id,&pool.get().unwrap()) {
        Ok(_) => {
            Ok(String::from("删除成功"))
        }
        Err(_) =>{
            Ok(String::from("删除失败请稍后再试"))
        } 
    }
}



pub async fn sync(Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    match Video::sync(&pool.get().unwrap()) {
        Ok(result) => {
            Ok(String::from(result))
        }
        Err(e) =>{
            Ok(String::from(format!("同步失败:{}",e)))
            //Ok(String::from("同步失败请稍后再试"))
        } 
    }  

}

pub async fn add_task(Query(params): Query<HashMap<String, String>>,Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    let download_url = match params.get("download_url") {
        Some(url) => url,
        None => {
            return Ok(String::from("下载地址不能为空"))
        }
    };

    if download_url.trim().len() == 0 {
        return Ok(String::from("下载地址不能为空"))
    }

    let isnow = match params.get("isnow") {
        Some(isnow) => isnow.parse::<i32>().unwrap(),
        None => {
            0
        }
    };
        
    match Video::add_task(&download_url,isnow,&pool.get().unwrap()) {
        Ok(result) => {
            Ok(format!("添加{}任务成功",result))
        }
        Err(_) =>{
            Ok(String::from("添加任务失败请稍后再试"))
        } 
    }
}

pub async fn create_m3u(Extension(save_dir): Extension<String>,Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    debug!("{}", save_dir);
    let m3u_path = format!("{}/{}", save_dir, "videos.m3u");
    let gateway = match Setting::find_value_by_key(&"gateway".to_string(), &pool.get().unwrap()) {
        Ok(gateway) => gateway,
        Err(err) => {
            debug!("{:?}", err);
            "".to_string()
        }
    };
    let videos = match Video::get_all(&pool.get().unwrap()) {
        Ok(result) => {
            result
        }
        Err(_) =>{
            return Ok(String::from("获取视频列表失败"))
        } 
    };

    if gateway.trim().len() == 0 {
        return Ok(String::from("请先设置gateway"))
    }

    let mut m3u_contents = String::from("#EXTM3U\n");
    for video in videos {
        let video_url = format!("{}/{}/{}",gateway,video.cid,video.name);
        let video_img = match video.img {
            Some(img) => img,
            None => "".to_string()
        };
        m3u_contents.push_str(&format!("#EXTINF:-1 tvg-logo=\"{}\",{}\n{}\n",video_img,video.title,video_url));
    }

    let mut file = File::create(m3u_path).unwrap();
    file.write_all(m3u_contents.as_bytes()).unwrap();

    Ok("生成完成".to_string())
}
