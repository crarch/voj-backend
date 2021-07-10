use mongodb::bson::doc;
use bson::document::Document;

use crate::MongoDB;
pub async fn query_testbench_by_id(mongo:MongoDB,question_id:u32)->Result<Document,()>{
    let collection=mongo.collection::<Document>("testbenches");

    if let Ok(cursor)=collection.find_one(
        doc!{"_id":question_id},
        None,
    ).await{
        if let Some(result)=cursor{
            return Ok(result);
        }
    }
        
    Err(())
    
}

pub async fn query_testbench_update_by_id(mongo:MongoDB,question_id:u32)->Result<u32,()>{
    let collection=mongo.collection::<Document>("testbenches");

    if let Ok(cursor)=collection.find_one(doc!{"_id":question_id},None).await{
        if let Some(result)=cursor{
            if let Ok(result)=result.get_i32("update"){
                return Ok(result as u32);
            }
        }
    }

    Err(())

}
