use actix_web::{web,HttpResponse,post,Responder,Error,get,delete};
use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::Pool;
use crate::models::user::User;
use crate::utils::auth::UserAuthJson;
use crate::env::get_env;



#[post("/session")]
pub async fn get_jwt_token(
    pool:web::Data<Pool>,
    user_auth_json:web::Json<UserAuthJson>
)->Result<HttpResponse,Error>{
    
    Ok(HttpResponse::Ok().body("ok"))
}



