#![allow(unused_must_use)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;


use std::{
  net::{ToSocketAddrs},
  convert::Infallible,
  fs,
  env
};
use structopt::StructOpt;
use tracing::{info,Level};
use tracing_subscriber::FmtSubscriber;
use webdav_handler::{fakels::FakeLs, localfs::LocalFs,DavHandler};


mod config;
mod utils;
mod controllers;
mod routes;
mod db;
mod constants;
mod models;
mod schema;
mod middlewares;


#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
      env::set_var("RUST_LOG", "axum-web=info");
    }
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let config = config::env::ServerConfig::from_args();
    let wevdav_app = async {
        let webdav_config = config.clone();
        let dir = webdav_config.root;
        fs::create_dir_all(format!("{}/music-data/mp3",&dir.to_string()));
        fs::create_dir_all(format!("{}/music-data/cover",&dir.to_string()));
        fs::create_dir_all(format!("{}/music-data/lrc",&dir.to_string()));
        let webdav_root = format!("{}/music-data",&dir.to_string());

        let addr = (webdav_config.host, webdav_config.webdav_port)
                    .to_socket_addrs()
                    .unwrap()
                    .next()
                    .unwrap();
        let dav_server = DavHandler::builder()
            .filesystem(LocalFs::new(webdav_root, false, false, false))
            .locksystem(FakeLs::new())
            .autoindex(true)
            .build_handler();
    
        let make_service = hyper::service::make_service_fn(move |_| {
            let dav_server = dav_server.clone();
            async move {
                let func = move |req| {
                    let dav_server = dav_server.clone();
                    async move { Ok::<_, Infallible>(dav_server.handle(req).await) }
                };
                Ok::<_, Infallible>(hyper::service::service_fn(func))
            }
        });

        info!("Webdav服务端口:ttp://:{:?}", addr);
        let _ = hyper::Server::bind(&addr)
            .serve(make_service)
            .await
            .map_err(|e| eprintln!("server error: {}", e));

    };

    let web_app = async {
        let web_config = config.clone();
        let app = routes::create_router(&web_config);
        let addr = (web_config.host, web_config.port)
                    .to_socket_addrs()
                    .unwrap()
                    .next()
                    .unwrap();
        info!("请在浏览器上打开http://:{:?}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    };

    tokio::join!(wevdav_app, web_app);

}

