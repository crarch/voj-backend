

use mongodb::bson::doc;
use bson::oid::ObjectId;


use serde_json::Value;
use bson::Bson;

use crate::utils::time::get_unix_timestamp;
use crate::MongoDB;
use crate::actors::push_job;
use crate::Queue;

pub async fn add_job(
    _mongo:MongoDB,
    queue:Queue,
    question_id:u32,
    update:u32,
    user_id:u32,
    code:&str,
)->Result<ObjectId,()>{
    
    let object_id=ObjectId::new();
    
    let doc=doc!{
        "_id":object_id,
        "user_id":user_id,
        "question_id":question_id,
        "update":update,
        "submit_time":get_unix_timestamp(),
        "code":code,
    };
    
    let job:Bson=Bson::from(doc.clone()).into();
    let job:Value=job.into();
    let job=job.to_string();
    
    push_job(queue,job).await;
    
    return Ok(object_id);
        
}

