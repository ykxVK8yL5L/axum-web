use std::fs;
use crate::utils::{template::{HtmlTemplate,OfflineTemplate}};
use axum::{
    extract::Form,
    response::{IntoResponse},
};



pub async fn offline_home() -> impl IntoResponse {
    let name = "离线下载".to_string();
    //let auth_info = fs::read_to_string("setting.json").expect("Something went wrong reading the file");

    let auth_info = match fs::read_to_string("setting.json") {
        Err(e) => "{}".to_string(),
        Ok(f) => f,
    };


    let offline_template = OfflineTemplate { name, auth_info };
    HtmlTemplate(offline_template)
}


#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Input {
    pub setting: String,
}

pub async fn offline_save_auth(form: Form<Input>) -> String {
    let input: Input = form.0;
    // rustぽい書き方
    match fs::write("setting.json",input.setting) {
        Err(result) => format!("保存失败"),
        Ok(_) => format!("保存成功"),
    }

}

