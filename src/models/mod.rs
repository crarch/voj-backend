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


pub fn get_pass_by_id(mongo:MongoDB,user_id:u32)->Result<Pass,()>{
    let collection=mongo.collection::<Pass>("users");

    let cursor=collection.find_one(doc!{"_id":user_id},None).unwrap();

    let result=cursor.unwrap();
    
    Ok(result)
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


pub fn get_question_by_id(mongo:MongoDB,question_id:u32)->Result<Question,()>{
    let collection=mongo.collection::<Question>("questions");

    if let Ok(cursor)=collection.find_one(doc!{"_id":question_id},None){
        if let Some(result)=cursor{
            return Ok(result);
        }
    }
        
    Err(())
    
}

use bson::Bson;
#[derive(Debug,Serialize,Deserialize)]
pub struct Question{
    _id:u32,
    update:u64,
    test_bench:Bson,
}
