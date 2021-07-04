use actix_web::{web,HttpRequest,HttpResponse,post,Error,get};
use mongodb::bson::doc;

use crate::MongoDB;
use crate::models::UserId;

use crate::models::Pass;

#[get("/profile/pass")]
pub async fn get_pass(
    mongo:MongoDB,
    user_id:UserId
)->Result<HttpResponse,Error>{
    let collection=mongo.collection::<Pass>("pass");
    
    let cursor=collection.find_one(doc!{"user_id":1},None).unwrap();
    
    let result=cursor.unwrap();
    
    Ok(HttpResponse::Ok().json(result))
}