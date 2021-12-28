use axum::{
    Router,
    routing::{get},
};
use tower::{layer::layer_fn};
use crate::controllers::index::{root};
use crate::middlewares::auth_middleware::AuthMiddleware;

pub fn create_api_router()->Router{
    let api_router = Router::new()
            .route("/files", get(root))
            .layer(layer_fn(|inner| AuthMiddleware { inner }));
    api_router
}


