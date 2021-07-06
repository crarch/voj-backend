#![allow(unused_parens,dead_code)]
extern crate log;
extern crate env_logger;

mod routes;
mod models;
mod server;
mod database;
mod utils;
mod handlers;
mod middleware;

use crate::database::MongoDB;
use crate::utils::env;


#[actix_web::main]
async fn main()->std::io::Result<()>{
    println!("Verilog Online Judge {}",env::VERSION);
    
    server::server().await
}