use actix_web::web;

use mongodb::sync::Client;
pub type MongoDB=web::Data<mongodb::sync::Database>;

use crate::env::get_env;


pub fn get_mongo_database()->mongodb::sync::Database{
    
    let mongo_url=get_env("MONGODB_URL");
    let mongo_dbname=get_env("MONGODB_DBNAME");
    
    let client=Client::with_uri_str(&mongo_url).unwrap();
    let database=client.database(&mongo_dbname);
    
    database
}
    
    
    
        
