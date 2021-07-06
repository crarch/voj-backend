use serde::{Deserialize,Serialize};
use bson::document::Document;
use mongodb::bson::doc;
use bson::oid::ObjectId;

use crate::utils::time::get_unix_timestamp;
use crate::MongoDB;

pub fn queue_add_job(
    mongo:MongoDB,
    object_id:&str,
    question_id:u32,
    update:u32,
    code:&str
)->Result<(),()>{
    let collection=mongo.collection::<Document>("queue");
    
    let doc=doc!{
        "_id":ObjectId::parse_str(object_id).unwrap(),
        "question_id":question_id,
        "update":update,
        "submit_time":get_unix_timestamp(),
        "code":code,
        "judger":0u32
    };
    
    let result=collection.insert_one(doc,None);
    
    match result{
        Ok(_)=>{
            Ok(())
        },
        Err(_)=>Err(()),
    }
}

pub fn queue_get_first_job(
    mongo:MongoDB,
    judger_id:u32
)->Result<Document,()>{
    let collection=mongo.collection::<Document>("queue");

    if let Ok(_)=collection.update_one(
        doc!{"judger":0u32},
        doc!{"$set":{"judger":judger_id}},
        None
    ){
            return queue_get_job_by_id(mongo,judger_id);
    }
    
    Err(())
    
}

    
    
fn queue_get_job_by_id(
    mongo:MongoDB,
    judger_id:u32,
)->Result<Document,()>{
    let collection=mongo.collection::<Document>("queue");

    if let Ok(cursor)=collection.find_one(
        doc!{"judger":judger_id},
        None
    ){
        if let Some(result)=cursor{
            return Ok(result);
        }
    }
        
    Err(())
}
