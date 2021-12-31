use axum::{
    Router,
    routing::{get},
};
use tower::{layer::layer_fn};
use crate::controllers::index::{root};
use crate::middlewares::auth_middleware::AuthMiddleware;
use crate::{config::env::ServerConfig};

pub fn create_api_router(config:&ServerConfig)->Router{
    let api_router = Router::new()
            .route("/files", get(root))
            .layer(layer_fn(|inner| AuthMiddleware { inner }));
    api_router
}


