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
use crate::utils::template::{HtmlTemplate,VideoTemplate};
use crate::{
    db::Pool,
    models::{
      response::ResponseBody,
      videos::{Video},
      settings::{Setting},
    },
    constants,
};


pub async fn videos_home(Extension(pool): Extension<Pool>,) -> impl IntoResponse {
    let name = "IPFS文件管理".to_string();
    let gateway = match Setting::find_value_by_key(&"gateway".to_string(), &pool.get().unwrap()) {
        Ok(gateway) => gateway,
        Err(err) => {
            info!("{:?}", err);
            "".to_string()
        }
    };
    let hellotemplate =VideoTemplate { name,gateway };
    HtmlTemplate(hellotemplate)
}


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
