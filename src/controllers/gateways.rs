use std::collections::HashMap;
use axum::{
    Json,
    extract::{Query,Extension},
    http::{StatusCode},
};
use crate::{
    db::Pool,
    models::{
        gateways::{Gateway,GatewayDTO},
        settings::{Setting},
    },
};

pub async fn all(Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    match Gateway::get_all(&pool.get().unwrap()) {
        Ok(result) => {
            Ok(serde_json::to_string(&result).unwrap())
        }
        Err(_) =>{
            Ok(String::from("not ok"))
        } 
    }
}


pub async fn add(Extension(pool): Extension<Pool>,Json(gateway_dto): Json<GatewayDTO>,) -> Result<String, (StatusCode, String)> {
    match Gateway::insert(gateway_dto,&pool.get().unwrap()) {
        Ok(_) => {
            Ok(String::from("成功添加"))
        }
        Err(_) =>{
            Ok(String::from("添加失败请稍后再试"))
        } 
    }
}

pub async fn del(Query(params): Query<HashMap<String, String>>,Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    let gateway_url = match params.get("url") {
        Some(url) => url,
        None => {
            return Ok(String::from("url is empty"))
        }
    };
    match Gateway::delete(gateway_url,&pool.get().unwrap()) {
        Ok(_) => {
            Ok(String::from("删除成功"))
        }
        Err(_) =>{
            Ok(String::from("删除失败请稍后再试"))
        } 
    }
}

pub async fn save(Query(params): Query<HashMap<String, String>>,Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    let gateway_url = match params.get("url") {
        Some(url) => url,
        None => {
            return Ok(String::from("url is empty"))
        }
    };
    let key_name = "gateway".to_string();
    match Setting::update_value_by_key(&key_name,&gateway_url,&pool.get().unwrap()) {
        Ok(_) => {
            Ok(String::from("保存成功"))
        }
        Err(_) =>{
            Ok(String::from("保存失败请稍后再试"))
        } 
    }
}




