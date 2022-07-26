use std::str;
use std::collections::HashMap;
use axum::{
    body::Body,
    Json,
    extract::{Query,Extension},
    http::{Request, StatusCode},
    response::{IntoResponse},
};
use tracing::info;

use crate::{
    db::Pool,
    models::{
      response::ResponseBody,
      videos::{Video},
    },
    constants,
};



pub async fn videos_all(Query(params): Query<HashMap<String, String>>,Extension(pool): Extension<Pool>,) -> Result<String, (StatusCode, String)> {
    //info!("{}", params.get("search[value]").unwrap());
    match Video::pagination(&params,&pool.get().unwrap()) {
        Ok(result) => {
            Ok(result)
        }
        Err(_) =>{
            Ok(String::from("not ok"))
        } 
    }
}
