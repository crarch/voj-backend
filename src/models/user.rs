use serde::{Deserialize,Serialize};
use mongodb::bson::doc;
use bson::document::Document;

use crate::MongoDB;
    
pub async fn query_user_password_by_email(
    mongo:MongoDB,
    user_email:&str
)->Result<(u32,String),()>{
    
    let collection=mongo.collection::<UserPass>("users");
    
    let cursor=collection.find_one(doc!{"user_email":&user_email},None).await.unwrap();
    
    if let Some(result)=cursor{
        return Ok((result._id,result.user_password));
    }
    
    Err(())
}


pub async fn query_user_profile(
    mongo:MongoDB,
    user_id:u32
)->Result<Document,()>{
    
    let collection=mongo.collection::<Document>("users");
    
    let cursor=collection.find_one(
        doc!{"user_id":user_id},
        
        mongodb::options::FindOneOptions::builder()
            .projection(Some(doc!{"pass":0,"_id":1,"user_email":1}))
            .build()

    ).await.unwrap();
    
    return Ok(cursor.unwrap());
}

#[derive(Debug,Serialize,Deserialize)]
struct UserPass{
    _id:u32,
    user_password:String
}
