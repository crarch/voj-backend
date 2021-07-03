use actix_web::{web,HttpResponse,post,Responder,Error,get,delete};
use anyhow::Result;



use crate::Pool;

use crate::utils::auth::UserAuthJson;




#[post("/session")]
pub async fn get_jwt_token(
    _pool:web::Data<Pool>,
    _user_auth_json:web::Json<UserAuthJson>
)->Result<HttpResponse,Error>{
    
    Ok(HttpResponse::Ok().body("ok"))
}



