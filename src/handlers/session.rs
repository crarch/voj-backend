use actix_web::{web,HttpResponse,post,Responder,Error,get,delete};
use anyhow::Result;

use crate::Pool;
use crate::models::user::{User,UserAuthJson};


#[post("/session")]
pub async fn get_jwt_token(
    pool:web::Data<Pool>,
    user_auth_json:web::Json<UserAuthJson>
)->Result<HttpResponse,Error>{
    
    println!("{:?}",user_auth_json);
        
                
    Ok(HttpResponse::Ok().body("ok"))
}


