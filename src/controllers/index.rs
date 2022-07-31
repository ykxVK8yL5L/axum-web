use crate::utils::template::{HtmlTemplate,HelloTemplate,PasswordGeneratorTemplate};
use axum::{
    extract,
    response::{IntoResponse},
};



pub async fn root() -> &'static str {
    "Hello, World!"
}



pub async fn greet(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
  let hellotemplate =HelloTemplate { name };
  HtmlTemplate(hellotemplate)
}


pub async fn password_generator() -> impl IntoResponse {
  let name="密码生成器".to_string();
  let hellotemplate =PasswordGeneratorTemplate { name };
  HtmlTemplate(hellotemplate)
}





// ajax请求示例

// 前台ajax代码
// axios.post('/offline/save_auth', params,{headers: {'Content-Type': 'application/x-www-form-urlencoded'}})
// .then(function (response) {
//   //console.log(JSON.stringify(response.data));
//   var result = response.data;
//   layer.alert(result);
// })
// .catch(function (error) {
//   layer.alert("出现错误，稍后重试");
//   console.log(error);
// });

// #[derive(Deserialize, Debug)]
// #[allow(dead_code)]
// pub struct Input {
//     pub setting: String,
// }

// 后台ajax代码
// pub async fn offline_save_auth(form: Form<Input>) -> String {
//     let input: Input = form.0;
//     match fs::write("setting.json",input.setting) {
//         Err(result) => format!("保存失败"),
//         Ok(_) => format!("保存成功"),
//     }
// }

// ajax结束