

use mongodb::bson::doc;
use bson::oid::ObjectId;


use serde_json::Value;
use bson::Bson;

use crate::utils::time::get_unix_timestamp;
use crate::MongoDB;
use crate::actors::push_job;
use crate::Queue;
use crate::models::create_new_record;

pub async fn add_job(
    mongo:MongoDB,
    queue:Queue,
    question_id:u32,
    update:u32,
    user_id:u32,
    code:&str,
)->Result<ObjectId,()>{
    
    if let Ok(object_id)=create_new_record(mongo.clone(),user_id,question_id,code).await{
        
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
    
    Err(())
}

