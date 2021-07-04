use serde::{Deserialize,Serialize};

use crate::models::user::get_user_password_by_email;
use crate::Pool;


#[derive(Debug,Serialize,Deserialize)]
pub struct UserAuthJson{
    pub user_email:String,
    pub user_password:String,
}

pub fn auth_user(
    pg_pool:Pool,
    user_email:&str,
    _user_password:&str
)->Result<u32,()>{
    let (user_id,user_password)=get_user_password_by_email(pg_pool,user_email)?;
    if(_user_password==user_password){
        Ok(user_id)
    }else{
        Err(())
    }
}
        
        
        