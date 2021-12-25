use crate::utils::template::{HtmlTemplate,HelloTemplate};
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
