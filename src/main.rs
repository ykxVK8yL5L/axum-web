use std::net::{SocketAddr, ToSocketAddrs};
use std::{env,time::Duration};
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
use tower_http::{services::ServeDir, trace::TraceLayer};
use mime_guess;
use rust_embed::RustEmbed;
use structopt::StructOpt;
use tracing::{info,debug,error,Level};
use tracing_subscriber::FmtSubscriber;

#[derive(StructOpt, Debug)]
#[structopt(name = "axum-web")]
struct Opt {
    /// Listen host
    #[structopt(long, env = "HOST", default_value = "0.0.0.0")]
    host: String,
    /// Listen port
    #[structopt(short, env = "PORT", long, default_value = "10099")]
    port: u16,
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
      env::set_var("RUST_LOG", "axum-web=info");
    }
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let app = Router::new()
        .route("/greet/:name", get(greet))
        .route("/assets/", static_handler.into_service())
        .fallback(static_handler.into_service())
        .route("/", get(root));
    // run it
    let opt = Opt::from_args();
    let addr = (opt.host, opt.port)
              .to_socket_addrs()
              .unwrap()
              .next()
              .unwrap();
    info!("请在浏览器上打开http://:{:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn root() -> &'static str {
    "Hello, World!"
}
#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
}
async fn greet(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
  let template = HelloTemplate { name };
  HtmlTemplate(template)
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
    StaticFile(path)
}
  
#[derive(RustEmbed)]
#[folder = "public/assets"]
struct Asset;

pub struct StaticFile<T>(pub T);
 




#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
  label: String,
  message:String,
}

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
    match Asset::get(path.as_str()) {
      Some(content) => {
        let body = boxed(Full::from(content.data));
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        Response::builder().header(header::CONTENT_TYPE, mime.as_ref()).body(body).unwrap()
      }
      None => {
        error!("{} 404 not found ",fullpath.as_str());
        //Response::builder().status(StatusCode::NOT_FOUND).body(boxed(Full::from("<h1>404</h1>"))).unwrap()
        let template = ErrorTemplate {label:"404 Not Found".to_string(),message:"没有找到相关的页面信息，请确保路径正确！".to_string()};
        HtmlTemplate(template).into_response()
      }
    }
  }
}
