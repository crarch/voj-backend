use actix_web::{FromRequest,HttpRequest};
use actix_web::dev::Payload;
use futures_util::future::{ok,err};

pub struct UserId{
    pub user_id:u32,
}

impl FromRequest for UserId {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();


    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        match req.extensions().get::<UserId>() {
            Some(user) =>{
                let new_user_id=UserId{user_id:user.user_id};
                ok(new_user_id)
            },      
            None => err(actix_web::error::ErrorBadRequest(""))
        }
    }

}
