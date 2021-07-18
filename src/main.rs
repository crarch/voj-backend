#![allow(unused_parens,dead_code)]

mod routes;
mod models;
mod server;
mod database;
mod utils;
mod handlers;
mod middleware;
mod actors;

use crate::database::MongoDB;
use crate::utils::env;

use crate::server::Queue;


#[actix_web::main]
async fn main()->std::io::Result<()>{
    println!("Verilog Online Judge {}",env::VERSION);
    
    server::server().await
}