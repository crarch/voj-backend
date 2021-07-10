use actix_web::web;

// use mongodb::sync::Client;
pub type MongoDB=web::Data<mongodb::Database>;

use crate::env::get_env;


// pub fn get_mongo_database()->mongodb::sync::Database{
// 
//     let mongo_url=get_env("MONGODB_URL");
//     let mongo_dbname=get_env("MONGODB_DBNAME");
// 
//     let client=Client::with_uri_str(&mongo_url).unwrap();
//     let database=client.database(&mongo_dbname);
// 
//     database
// }
    
    
use lazy_static::lazy_static;
use mongodb::{Client, options::ClientOptions};


pub async fn get_db() -> mongodb::Database {
    let mongo_url=get_env("MONGODB_URL");
    let client_options=ClientOptions::parse(&mongo_url).await.unwrap();
    let client=Client::with_options(client_options).unwrap();
    let mongo_db_name=get_env("MONGODB_DBNAME");
    let db=client.database(&mongo_db_name);
    db    
}
        
