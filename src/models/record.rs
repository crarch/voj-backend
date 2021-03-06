
use bson::document::Document;
use mongodb::bson::doc;
use bson::oid::ObjectId;
use futures_util::TryStreamExt;


use crate::MongoDB;



pub async fn query_record_paging(mongo:MongoDB,user_id:u32)->Result<Document,()>{
    
    let collection=mongo.collection::<Document>("records");
    
    if let Ok(result)=collection.count_documents(
        doc!{"user_id":user_id},
        None
    ).await{
        let result=doc!{
            "default_page_size":20u32,
            "total_records":result
        };
        return Ok(result);
    }
    
    Err(())

    
}



//only owner can access record
pub async fn query_record_by_object_id(mongo:MongoDB,object_id:ObjectId,user_id:u32)->Result<Document,()>{
    
    let collection=mongo.collection::<Document>("records");
    
    if let Ok(result)=collection.find_one(
        doc!{"_id":object_id,"user_id":user_id},
        None
    ).await{
        if let Some(result)=result{
            return Ok(result);
        }
    }
    
    Ok(doc!{})
}

pub async fn query_record_list_by_page(mongo:MongoDB,page:u64,user_id:u32)->Result<Vec<Document>,()>{
    
    let collection=mongo.collection::<Document>("records");
    let page=(page-1)*20;
    if let Ok(mut cursor)=collection.find(
        doc!{"user_id":user_id},
        mongodb::options::FindOptions::builder()
            .projection(Some(doc!{
                "user_id":1,
                "submit_time":1,
                "question_id":1,
                "success":1
            }))
            .sort(doc!{"submit_time":-1})
            .limit(Some(20))
            .skip(Some(page))
            .build()
    ).await{
        let mut result=Vec::new();
        while let Some(record)=cursor.try_next().await.unwrap(){
            result.push(record);
        }
        return Ok(result);
    }
    
    Err(())
}

pub async fn query_record_list_by_page_and_question(mongo:MongoDB,question_id:u32,page:u64,user_id:u32)->Result<Vec<Document>,()>{
    
    let collection=mongo.collection::<Document>("records");
    let page=(page-1)*20;
    if let Ok(mut cursor)=collection.find(
        doc!{"user_id":user_id,"question_id":question_id},
        mongodb::options::FindOptions::builder()
            .projection(Some(doc!{
                "user_id":1,
                "submit_time":1,
                "question_id":1,
                "success":1
            }))
            .sort(doc!{"submit_time":-1})
            .limit(Some(20))
            .skip(Some(page))
            .build()
    ).await{
        let mut result=Vec::new();
        while let Some(record)=cursor.try_next().await.unwrap(){
            result.push(record);
        }
        return Ok(result);
    }
    
    Err(())
}
