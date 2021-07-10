use actix_web::web;
use mongodb::{Client, options::ClientOptions};

use crate::env::get_env;

pub type MongoDB=web::Data<mongodb::Database>;

pub async fn get_db() -> mongodb::Database {
    let mongo_url=get_env("MONGODB_URL");
    let client_options=ClientOptions::parse(&mongo_url).await.unwrap();
    let client=Client::with_options(client_options).unwrap();
    let mongo_db_name=get_env("MONGODB_DBNAME");
    let db=client.database(&mongo_db_name);
    db    
}
        
