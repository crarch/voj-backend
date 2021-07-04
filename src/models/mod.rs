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


impl Pass{
    pub fn get_pass_by_id(mongo:MongoDB,user_id:u32)->Pass{
        let collection=mongo.collection::<Pass>("pass");

        let cursor=collection.find_one(doc!{"user_id":user_id},None).unwrap();

        let result=cursor.unwrap();
        
        result
    }
}
