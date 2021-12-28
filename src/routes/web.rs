use axum::{
    Router,
    routing::{get},
};
use crate::controllers::{
    index::{root,greet},
};


pub fn create_web_router()->Router{
    let web_router = Router::new()
            .route("/greet/:name", get(root))
            .route("/", get(root));
    web_router
}


