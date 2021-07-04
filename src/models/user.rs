use serde::{Deserialize,Serialize};
use mongodb::bson::doc;

use crate::MongoDB;
    
pub fn get_user_password_by_email(
    mongo:MongoDB,
    user_email:&str
)->Result<(u32,String),()>{
    
    let collection=mongo.collection::<UserPass>("users");
    
    let cursor=collection.find_one(doc!{"user_email":&user_email},None).unwrap();
    
    if let Some(result)=cursor{
        return Ok((result._id,result.user_password));
    }
    
    Err(())
}

#[derive(Debug,Serialize,Deserialize)]
struct UserPass{
    _id:u32,
    user_password:String
}
