use axum::{
    Router,
    routing::{post},
};
use crate::controllers::auth::{signup,login};


pub fn create_auth_router()->Router{
    let auth_router = Router::new()
            .route("/login", post(login))
            .route("/signup", post(signup));
    auth_router
}


