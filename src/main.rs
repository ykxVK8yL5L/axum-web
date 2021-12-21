use askama::Template;
use axum::{
    body::{boxed,self, Full},
    extract,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get_service,get},
    Router,
    handler::Handler,
};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};
use mime_guess;
use rust_embed::RustEmbed;




#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_templates=debug")
    }
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/greet/:name", get(greet))
        .route("/assets/", static_handler.into_service())
        .fallback(static_handler.into_service())
        .route("/", get(root));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 10099));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn greet(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}


async fn root() -> &'static str {
    "Hello, World!"
}



#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
}

struct HtmlTemplate<T>(T);

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

// static_handler is a handler that serves static files from the
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.starts_with("assets/") {
      path = path.replace("assets/", "");
    }
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
      let path = self.0.into();
      match Asset::get(path.as_str()) {
        Some(content) => {
          let body = boxed(Full::from(content.data));
          let mime = mime_guess::from_path(path).first_or_octet_stream();
          Response::builder().header(header::CONTENT_TYPE, mime.as_ref()).body(body).unwrap()
        }
        None => Response::builder().status(StatusCode::NOT_FOUND).body(boxed(Full::from("404"))).unwrap(),
      }
    }
  }
