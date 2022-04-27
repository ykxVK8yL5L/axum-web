use crate::utils::{template::{HtmlTemplate,OfflineTemplate}};
use axum::{
    response::{IntoResponse},
};



pub async fn offline_home() -> impl IntoResponse {
    let name = "离线下载".to_string();
    let offline_template = OfflineTemplate { name };
    HtmlTemplate(offline_template)
}








