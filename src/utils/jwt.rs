use serde::{Deserialize, Serialize};

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::env::get_env;

#[derive(Debug,Serialize,Deserialize)]
struct Claims{
    user_id:i32,
    expiration:usize,
}

pub fn sign_jwt(user_id:i32)->Result<String,()>{
    let my_claims=Claims{
        user_id:1,
        expiration:100000000,
    };
    let key=get_env("SECRET_KEY");
    
    let key=key.as_bytes();
    
    let mut header=Header::default();
    
    header.kid = Some("signing_key".to_owned());
    header.alg = Algorithm::HS512;
    
    match encode(&header, &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => Ok(t),
        Err(_) =>Err(()), 
    }
}
