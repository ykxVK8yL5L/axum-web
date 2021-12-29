use axum::{
	response::Response,
    body::{Body,self,Full},
    http::{Request,Method,StatusCode},
    http::header::{HeaderName, HeaderValue},
};
use futures::future::BoxFuture;
use tower::{Service};
use std::task::{Context, Poll};
use crate::{constants,models::response::ResponseBody,db::Pool,utils::token_utils};


#[derive(Clone)]
pub struct AuthMiddleware<S> {
    pub inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let mut authenticate_pass: bool = false;
        let headers = req.headers_mut();
        headers.append(HeaderName::from_static("content-length"),HeaderValue::from_static("true"));
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            if !authenticate_pass {

                if let Some(pool) = req.extensions().get::<Pool>() {
                    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
                        if let Ok(authen_str) = authen_header.to_str() {
                            if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                                let token = authen_str[6..authen_str.len()].trim();
                                if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                                    if token_utils::verify_token(&token_data,pool).is_ok() {
                                        authenticate_pass = true;
                                    } else {
                                        authenticate_pass = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }



        if authenticate_pass {
            println!("`has authed` authenticate_pass called true!");
            let clone = self.inner.clone();
            let mut inner = std::mem::replace(&mut self.inner, clone);
            Box::pin(async move {
                let res: Response = inner.call(req).await?;
                println!("`MyMiddleware` received the response");

                Ok(res)
            })
        }else {
            println!("`authed` authenticate_pass called false!");

            Box::pin(async move {
                let response_body = ResponseBody::new(constants::MESSAGE_INVALID_TOKEN, constants::EMPTY);
                let response = serde_json::to_string(&response_body).unwrap();
                Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(body::boxed(Full::from(serde_json::to_string(&response_body).unwrap())))
                    .unwrap()
                )
            })
        }

    }
}




