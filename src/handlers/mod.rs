pub mod session;

use crate::env;

use actix_web::{web,HttpResponse,post,Responder,Error,get,delete};
use anyhow::Result;

use crate::models::UserId;






#[get("/version")]
pub async fn get_version(user_id:UserId)->Result<HttpResponse,Error>{
    println!("{}",user_id.user_id.to_string());
    Ok(HttpResponse::Ok().body(format!("{{\"version\":\"{}\"}}",env::VERSION)))
}