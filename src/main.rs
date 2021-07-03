#![allow(unused_parens)]



#[macro_use]
extern crate log;
extern crate env_logger;

mod routes;
mod models;
mod server;
mod database;
mod utils;
mod handlers;

use crate::database::Pool;
use crate::utils::env;

#[actix_web::main]
async fn main()->std::io::Result<()>{
    println!("Verilog Online Judge {}",env::VERSION);
    
    server::server().await
}