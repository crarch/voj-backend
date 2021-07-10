use actix_web::{HttpResponse,Error,get};

use crate::models::UserId;
use crate::MongoDB;

use crate::models::query_pass_by_id;

#[get("/profile/pass")]
pub async fn get_pass(
    mongo:MongoDB,
    user_id:UserId
)->Result<HttpResponse,Error>{
    
    let result=query_pass_by_id(mongo,user_id.user_id).await.unwrap();
    
    Ok(HttpResponse::Ok().json(result))
}