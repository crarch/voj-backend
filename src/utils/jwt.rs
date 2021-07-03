use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,TokenData};

use crate::env::get_env;

#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
    user_id:u32,
    exp:u64
}

impl Claims{
    pub fn get_user_id(&self)->u32{
        self.user_id
    }
}
        

pub fn sign_jwt(user_id:u32)->Result<String,()>{
    let my_claims=Claims{
        user_id:user_id,
        exp:now_plus_days(1),
    };
    let key=get_env("SECRET_KEY");
    
    let key=key.as_bytes();
    
    match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => {
            Ok(t)
        }
        Err(_) =>Err(()), 
    }

}

pub fn validate_jwt(token:&str)->Result<TokenData<Claims>,jsonwebtoken::errors::Error>{
    let key=get_env("SECRET_KEY");
    let key=key.as_bytes();
    decode::<Claims>(token, &DecodingKey::from_secret(key), &Validation::default())
}

fn now_plus_days(days: u64) -> u64 {
    let result=SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;
    let result=result+days*1000*24*60*60;
    result
}