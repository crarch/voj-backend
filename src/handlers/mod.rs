pub mod session;

use crate::env;


use actix_web::{web,HttpResponse,post,Responder,Error,get,delete};
use anyhow::Result;

use crate::Pool;
use crate::models::user::User;

#[get("/version")]
pub async fn get_version(
)->Result<HttpResponse,Error>{
    Ok(HttpResponse::Ok().body(format!("{{\"version\":\"{}\"}}",env::VERSION)))
}