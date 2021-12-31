use axum::{
    Router,
    routing::{get,post},
};
use crate::controllers::auth::{signup,login,logout};
use crate::{config::env::ServerConfig};

pub fn create_auth_router(config:&ServerConfig)->Router{
    let auth_router = Router::new()
            .route("/login", post(login))
            .route("/logout", get(logout))
            .route("/signup", post(signup));
    auth_router
}


