use serde::{Deserialize,Serialize};

use crate::Pool;
use actix_web::web;

#[derive(Debug,Serialize,Deserialize)]
pub struct UserAuthJson{
    pub user_email:String,
    pub user_password:String,
}
