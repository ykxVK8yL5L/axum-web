use std::collections::HashMap;
use axum::{
    extract::{Query,Extension},
    http::{StatusCode},
};
use crate::{
    db::Pool,
    models::{
        settings::{Setting},
    },
};

pub async fn all(Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    match Setting::get_all(&pool.get().unwrap()) {
        Ok(result) => {
            Ok(serde_json::to_string(&result).unwrap())
        }
        Err(_) =>{
            Ok(String::from("not ok"))
        } 
    }
}




pub async fn save(Query(params): Query<HashMap<String, String>>,Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    let setting_key = match params.get("key") {
        Some(key) => key,
        None => {
            return Ok(String::from("Key is empty"))
        }
    };

    let setting_value = match params.get("value") {
        Some(value) => value,
        None => {
            return Ok(String::from("Value is empty"))
        }
    };
    
    match Setting::update_value_by_key(&setting_key,&setting_value,&pool.get().unwrap()) {
        Ok(_) => {
            Ok(String::from("保存成功"))
        }
        Err(_) =>{
            Ok(String::from("保存失败请稍后再试"))
        } 
    }
}




