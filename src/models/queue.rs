use serde::{Deserialize,Serialize};
use bson::document::Document;
use mongodb::bson::doc;
use bson::oid::ObjectId;

use crate::utils::time::get_unix_timestamp;
use crate::models::pass::add_pass_by_id;
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
)->Result<Document,()>{
    let collection=mongo.collection::<Document>("queue");

    if let Ok(cursor)=collection.find_one(
        doc!{"lock_time":doc!{"$exists":false}},
        None
    ){
        if let Some(result)=cursor{
            if let Ok(object_id)=result.get_object_id("_id"){
                if let Ok(_)=lock_job_by_id(mongo,object_id){
                    return Ok(result);
                }
            }
        }
    }
        
    Err(())
    
}

fn lock_job_by_id(
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
    ){
        if(result.modified_count==1){
            return Ok(());
        }else{
            return Err(());
        }
    }
    
    Err(())
    
}
    


pub fn queue_delete_job_by_id(
    mongo:MongoDB,
    object_id:&str,
)->Result<(),()>{
    
    let collection=mongo.collection::<Document>("queue");
    
    if let Ok(object_id)=ObjectId::parse_str(object_id){
        if let Ok(_result)=collection.delete_one(
            doc!{"_id":object_id},
            None
        ){
            return Ok(());
        }
    }
    Err(())
}

pub fn queue_update_judge_result(
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

        ){
            
            if(is_success){
                let result=collection.find_one(
                    doc!{"_id":object_id},
                    mongodb::options::FindOneOptions::builder()
                        .projection(Some(doc!{"user_id":1,"question_id":1,"_id":0}))
                        .build()
                );
                
                let result=result.unwrap().unwrap();
                
                let user_id=result.get_i32("user_id").unwrap() as u32;
                let question_id=result.get_i32("question_id").unwrap() as u32;
                
                let _result=add_pass_by_id(mongo,user_id,question_id).unwrap();    
            }
                
                

                
            return Ok(());
        }
    }
    Err(())
}

pub fn check_dead_job(mongo:MongoDB){
    let collection=mongo.clone().collection::<Document>("queue");
    
    if let Ok(cursor)=collection.find(
        doc!{"lock_time":doc!{"$gt":get_unix_timestamp()-20}},
        mongodb::options::FindOptions::builder()
            .projection(Some(doc!{"_id":1}))
            .build()
    ){
        for result in cursor{
            if let Ok(doc)=result{
                if let Ok(object_id)=doc.get_object_id("_id"){
                    let time_out=doc!{
                        "time_out":"timeout",
                    };
                    
                    let object_id=object_id.to_hex();
                    
                    let _=queue_update_judge_result(
                        mongo.clone(),
                        &object_id,
                        false,
                        &time_out
                    ).unwrap();
                    
                    let _=queue_delete_job_by_id(
                        mongo.clone(),
                        &object_id,
                    ).unwrap();
                    
                }
            }
        }
    }
    
}

use std::thread;
use std::time::Duration;
    
pub fn cron(mongo:MongoDB){
    thread::spawn(move||
        loop{
            thread::sleep(Duration::from_secs(20));
            check_dead_job(mongo.clone());
        }
    );
    
}    
    
    
        
        
    
            

#[derive(Debug,Serialize,Deserialize)]
pub struct JudgeResultJson{
    pub _id:String,
    pub success:bool,
    pub test_bench:Document,
}
