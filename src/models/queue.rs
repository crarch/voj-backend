use serde::{Deserialize,Serialize};
use bson::document::Document;
use mongodb::bson::doc;
use bson::oid::ObjectId;
use futures_util::TryStreamExt;
use tokio::time::{self, Duration};

use crate::utils::time::get_unix_timestamp;
use crate::MongoDB;

pub async fn add_job(
    mongo:MongoDB,
    object_id:&str,
    question_id:u32,
    update:u32,
    user_id:u32,
    code:&str
)->Result<(),()>{
    let collection=mongo.collection::<Document>("queue");
    
    let doc=doc!{
        "_id":ObjectId::parse_str(object_id).unwrap(),
        "user_id":user_id,
        "question_id":question_id,
        "update":update,
        "submit_time":get_unix_timestamp(),
        "code":code,
    };
    
    let result=collection.insert_one(doc,None).await;
    
    match result{
        Ok(_)=>{
            Ok(())
        },
        Err(_)=>Err(()),
    }
}

pub async fn query_first_job(
    mongo:MongoDB,
)->Result<Document,()>{
    let collection=mongo.collection::<Document>("queue");

    if let Ok(cursor)=collection.find_one(
        doc!{"lock_time":doc!{"$exists":false}},
        None
    ).await{
        if let Some(result)=cursor{
            if let Ok(object_id)=result.get_object_id("_id"){
                if let Ok(_)=lock_job_by_id(mongo,object_id).await{
                    return Ok(result);
                }
            }
        }
    }
        
    Err(())
    
}

async fn lock_job_by_id(
    mongo:MongoDB,
    object_id:ObjectId
)->Result<(),()>{
    let collection=mongo.collection::<Document>("queue");
    
    if let Ok(result)=collection.update_one(
        doc!{"_id":object_id,"lock_time":doc!{"$exists":false}},
        doc!{
            "$set":{
                "lock_time":get_unix_timestamp()
            }
        },
        None
    ).await{
        if(result.modified_count==1){
            return Ok(());
        }else{
            return Err(());
        }
    }
    
    Err(())
    
}
    


pub async fn delete_job_by_id(
    mongo:MongoDB,
    object_id:&str,
)->Result<(),()>{
    
    let collection=mongo.collection::<Document>("queue");
    
    if let Ok(object_id)=ObjectId::parse_str(object_id){
        if let Ok(_result)=collection.delete_one(
            doc!{"_id":object_id},
            None
        ).await{
            return Ok(());
        }
    }
    Err(())
}

pub async fn update_judge_result(
    mongo:MongoDB,
    object_id:&str,
    is_success:bool,
    test_bench:&Document,
)->Result<(),()>{
    let collection=mongo.collection::<Document>("records");
    
    if let Ok(object_id)=ObjectId::parse_str(object_id){
        if let Ok(_result)=collection.update_one(
            doc!{"_id":object_id,"success":doc!{"$exists":false}},
            doc!{
                "$set":{
                    "success":is_success,
                    "test_bench":test_bench
                }
            },
            None

        ).await{
            return Ok(());
        }
    }
    Err(())
}

async fn check_dead_job(mongo:MongoDB){
    let collection=mongo.clone().collection::<Document>("queue");
    
    if let Ok(mut cursor)=collection.find(
        doc!{"lock_time":doc!{"$lt":get_unix_timestamp()-20}},
        mongodb::options::FindOptions::builder()
            .projection(Some(doc!{"_id":1}))
            .build()
    ).await{
        while let Some(doc)=cursor.try_next().await.unwrap(){
            if let Ok(object_id)=doc.get_object_id("_id"){
                let time_out=doc!{
                    "time_out":"timeout",
                };
                
                let object_id=object_id.to_hex();
                
                let _=update_judge_result(
                    mongo.clone(),
                    &object_id,
                    false,
                    &time_out
                ).await.unwrap();
                
                let _=delete_job_by_id(
                    mongo.clone(),
                    &object_id,
                ).await.unwrap();
                
            }
        }
    }
    
}

pub async fn cron(mongo:MongoDB){
    actix_rt::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(20));
        loop {
            interval.tick().await;
            check_dead_job(mongo.clone()).await;
        }
    });
}
    

#[derive(Debug,Serialize,Deserialize)]
pub struct JudgeResultJson{
    pub _id:String,
    pub success:bool,
    pub test_bench:Document,
    pub question_id:u32,
    pub user_id:u32
}
