use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{HttpResponse, ResponseError};
use futures::future::{ok, Ready};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::body::MessageBody;
use actix_web::HttpMessage;

use crate::utils::jwt::validate_jwt;
use crate::models::UserId;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {

            let mut authorized=false;
            if let Some(authorization)=req.headers().get("Authorization"){
                if let Ok(token)=authorization.to_str(){
                    if let Ok(token_data)=validate_jwt(token){
                        let user_id=token_data.claims.get_user_id();
                        let user_id=UserId{user_id:user_id};
                        req.extensions_mut().insert(user_id);
                        authorized=true;
                    }
                }
            }
        
            if(authorized){
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }else{
                Box::pin(async move {
                        Err(actix_web::error::ErrorUnauthorized("Invalid JWT Token"))
                })
            }
            
                    
    }
}
