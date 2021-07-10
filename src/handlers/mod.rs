// pub mod session;
// pub mod profile;
pub mod testbench;
// pub mod judge;
// pub mod queue;

use crate::env;

use actix_web::{HttpResponse,Error,get};
use anyhow::Result;

use crate::MongoDB;


#[get("/version")]
pub async fn get_version(mongo:MongoDB)->Result<HttpResponse,Error>{
    Ok(HttpResponse::Ok().body(format!("{{\"version\":\"{}\"}}",env::VERSION)))
}