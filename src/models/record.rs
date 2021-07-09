use serde::{Deserialize,Serialize};
use bson::document::Document;
use mongodb::bson::doc;
use bson::oid::ObjectId;

use crate::utils::time::get_unix_timestamp;
use crate::MongoDB;

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

pub fn get_record_list_by_page(mongo:MongoDB,page:u64,user_id:u32)->Result<Vec<Document>,()>{
    
    let collection=mongo.collection::<Document>("records");
    let page=(page-1)*20;
    if let Ok(cursor)=collection.find(
        doc!{"user_id":user_id},
        mongodb::options::FindOptions::builder()
            .projection(Some(doc!{
                "_id":1,
                "submit_time":1,
                "question_id":1,
                "success":1
            }))
            .sort(doc!{"submit_time":-1})
            .limit(Some(20))
            .skip(Some(page))
            .build()
    ){
        let mut result=Vec::new();
        for iter in cursor{
            result.push(iter.unwrap());
        }
        return Ok(result);
    }
    
    Err(())
}

pub fn get_record_list_by_page_and_question(mongo:MongoDB,question_id:u32,page:u64,user_id:u32)->Result<Vec<Document>,()>{
    
    let collection=mongo.collection::<Document>("records");
    let page=(page-1)*20;
    if let Ok(cursor)=collection.find(
        doc!{"user_id":user_id,"question_id":question_id},
        mongodb::options::FindOptions::builder()
            .projection(Some(doc!{
                "_id":1,
                "submit_time":1,
                "question_id":1,
                "success":1
            }))
            .sort(doc!{"submit_time":-1})
            .limit(Some(20))
            .skip(Some(page))
            .build()
    ){
        let mut result=Vec::new();
        for iter in cursor{
            result.push(iter.unwrap());
        }
        return Ok(result);
    }
    
    Err(())
}

#[derive(Debug,Serialize,Deserialize)]
pub struct CodeJson{
    pub question_id:u32,
    pub code:String
}
