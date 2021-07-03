use serde::{Deserialize,Serialize};




#[derive(Debug,Serialize,Deserialize)]
pub struct UserAuthJson{
    pub user_email:String,
    pub user_password:String,
}
