use serde::{Deserialize,Serialize};

use crate::models::user::query_user_password_by_email;
use crate::MongoDB;


#[derive(Debug,Serialize,Deserialize)]
pub struct UserAuthJson{
    pub user_email:String,
    pub user_password:String,
}

pub async fn auth_user(
    mongo:MongoDB,
    user_email:&str,
    _user_password:&str
)->Result<u32,()>{
    let (user_id,hash)=query_user_password_by_email(mongo,user_email).await?;
    if let Ok(matches)=argon2::verify_encoded(&hash,_user_password.as_bytes()){
        if(matches){
            return Ok(user_id);
        }
    }
    Err(())
}
        
        
        