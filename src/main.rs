#![allow(unused_must_use)]
#[macro_use]
extern crate diesel;
extern crate serde_derive;
extern crate serde_json;

use std::net::{ToSocketAddrs};
use std::{env};
use structopt::StructOpt;
use tracing::{info,Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod utils;
mod controllers;
mod routes;
mod db;
mod constants;
mod models;
mod schema;


#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
      env::set_var("RUST_LOG", "axum-web=info");
    }
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let app = routes::create_router();

  
    // run it
    let config = config::env::ServerConfig::from_args();
    let addr = (config.host, config.port)
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

