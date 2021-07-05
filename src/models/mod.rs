pub mod user;
pub mod user_id;

pub use user_id::UserId;

use serde::{Deserialize,Serialize};
use mongodb::bson::doc;

use crate::MongoDB;

#[derive(Debug,Serialize,Deserialize)]
pub struct Pass{
    pass:Vec<i32>,
}


pub fn get_pass_by_id(mongo:MongoDB,user_id:u32)->Pass{
    let collection=mongo.collection::<Pass>("users");

    let cursor=collection.find_one(doc!{"_id":user_id},None).unwrap();

    let result=cursor.unwrap();
    
    result
}

pub fn add_pass_by_id(mongo:MongoDB,user_id:u32,pass:u32)->Result<(),()>{
    let collection=mongo.collection::<Pass>("users");
    
    let result=collection.update_one(
        doc!{"_id":user_id},
        doc!{"$addToSet":{"pass":pass}},
        None,
    );
    
    match result{
        Ok(_)=>Ok(()),
        Err(_)=>Err(()),
    }
}
