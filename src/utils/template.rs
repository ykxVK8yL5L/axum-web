use askama::Template;
use axum::{
    body::{self, Full},
    http::{StatusCode},
    response::{Html, IntoResponse, Response},
};



#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate {
    pub name: String,
}


#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub name: String,
}


#[derive(Template)]
#[template(path = "password_generator.html")]
pub struct PasswordGeneratorTemplate {
    pub name: String,
}


#[derive(Template)]
#[template(path = "jsoneditor.html")]
pub struct JsonEditorTemplate {
    pub name: String,
}


#[derive(Template)]
#[template(path = "music.html")]
pub struct MusicTemplate {
    pub name: String,
}


#[derive(Template)]
#[template(path = "video.html")]
pub struct VideoTemplate {
    pub name: String,
    pub gateway: String,
}





#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub label: String,
    pub message:String,
}





pub struct HtmlTemplate<T>(pub T);
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body::boxed(Full::from(format!(
                    "Failed to render template. Error: {}",
                    err
                ))))
                .unwrap(),
        }
    }
}
