use axum::{
    error_handling::HandleErrorLayer,
    body::{boxed,Full},
    routing::{get},
    Router,
    http::{header, Uri,StatusCode},
    response::{IntoResponse, Response},
    handler::Handler,
};
use std::{
  time::Duration,
};
use rust_embed::RustEmbed;
use mime_guess;
use tracing::{error};
use crate::utils::template::{HtmlTemplate,ErrorTemplate};
use crate::controllers::{
  home
};
use crate::{db,config::env::ServerConfig};
use tower::{BoxError, ServiceBuilder};
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};


automod::dir!("src/routes");


pub fn create_router(config:&ServerConfig)-> Router{
    let pool = db::init_db(config.database.to_string());
    //let music_routes = music::create_music_router(config);
    // .nest("/web", web::create_web_router())
    // .nest("/api", api::create_api_router())
    // .nest("/auth", auth::create_auth_router())
    automod::create_controllers_route!("src/routes");
    let app = Router::new()
    .merge(empty_route)
    .route("/assets/", static_handler.into_service())
    .fallback(static_handler.into_service())
    .layer(
      ServiceBuilder::new()
      .layer(HandleErrorLayer::new(|error: BoxError| async move {
          if error.is::<tower::timeout::error::Elapsed>() {
              Ok(StatusCode::REQUEST_TIMEOUT)
          } else {
              Err((
                  StatusCode::INTERNAL_SERVER_ERROR,
                  format!("Unhandled internal error: {}", error),
              ))
          }
      }))
      .timeout(Duration::from_secs(20))
      .layer(TraceLayer::new_for_http())
      .into_inner(),
    )
    .layer(AddExtensionLayer::new(pool));
   
    app
}

// static_handler is a handler that serves static files from the
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}
  
#[derive(RustEmbed)]
#[folder = "public/assets"]
struct Asset;
pub struct StaticFile<T>(pub T);
impl<T> IntoResponse for StaticFile<T>
where
  T: Into<String>,
{
  fn into_response(self) -> Response {
    let mut path = self.0.into();
    let fullpath = path.clone();
    if path.starts_with("assets/") {
      path = path.replace("assets/", "");
    }
    if path.starts_with("favicon.ico") {
      path = "favicon.ico".to_string();
    }
    match Asset::get(path.as_str()) {
      Some(content) => {
        let body = boxed(Full::from(content.data));
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        Response::builder().header(header::CONTENT_TYPE, mime.as_ref()).body(body).unwrap()
      }
      None => {
        error!("{} 404 not found ",fullpath.as_str());
        //Response::builder().status(StatusCode::NOT_FOUND).body(boxed(Full::from("<h1>404</h1>"))).unwrap()
        let errortemplate = ErrorTemplate {label:"404 Not Found".to_string(),message:"没有找到相关的页面信息，请确保路径正确！".to_string()};
        HtmlTemplate(errortemplate).into_response()
      }
    }
  }
}
