pub mod user;
pub mod user_id;
pub mod pass;
pub mod question;
pub mod queue;

pub use pass::*;
pub use queue::*;
pub use question::*;
pub use user_id::UserId;

use serde::{Deserialize,Serialize};
use bson::document::Document;
use mongodb::bson::doc;
use bson::oid::ObjectId;

use crate::utils::time::get_unix_timestamp;
use crate::MongoDB;


#[derive(Debug,Serialize,Deserialize)]
pub struct Pass{
    pass:Vec<i32>,
}


pub fn create_new_record(mongo:MongoDB,user_id:u32,question_id:u32,code:&str)->Result<Document,()>{
    
    let collection=mongo.collection::<Document>("records");
    
    let doc=doc!{
        "user_id":user_id,
        "question_id":question_id,
        "submit_time":get_unix_timestamp(),
        "code":code
    };
    
    let result=collection.insert_one(doc,None);
    
    match result{
        Ok(insert_result)=>{
            let object_id=insert_result.inserted_id
                .as_object_id()
                .unwrap()
                .to_hex();
            
            let result=doc!{
                "$oid":object_id,
            };
            
            Ok(result)
        },
        Err(_)=>Err(()),
    }
    
}





//only owner can access record
pub fn get_record_by_object_id(mongo:MongoDB,object_id:&str,user_id:u32)->Result<Document,()>{
    
    let collection=mongo.collection::<Document>("records");
    
    if let Ok(object_id)=ObjectId::parse_str(object_id){
        if let Ok(result)=collection.find_one(
            doc!{"_id":object_id,"user_id":user_id},
            None
        ){
            if let Some(result)=result{
                return Ok(result);
            }
        }
    }    
    
    Err(())
}



#[derive(Debug,Serialize,Deserialize)]
pub struct CodeJson{
    pub question_id:u32,
    pub code:String
}
