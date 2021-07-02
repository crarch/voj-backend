use serde::{Deserialize,Serialize};

use crate::Pool;
use actix_web::web;

#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    pub user_id:i32,
    pub user_name:String,
    pub user_email:String,
    pub user_password:String,
    pub ban_login:bool,
    pub ban_judge:bool,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserAuthJson{
    pub user_email:String,
    pub user_password:String,
}


impl User{
    pub fn new(
        user_id:i32,
        user_name:String,
        user_email:String,
        user_password:String,
        ban_login:bool,
        ban_judge:bool,
    )->Result<User,()>{
        let new_user=User{
            user_id:user_id,
            user_name:user_name,
            user_email:user_email,
            user_password:user_password,
            ban_login:ban_login,
            ban_judge:ban_judge,
        };
        
        Ok(new_user)
    }
    
    
    
    pub fn get_user_by_id(
        pool:web::Data<Pool>,
        user_id:i32,
    )->Result<User,()>{
        let mut client=pool.get().unwrap();
        let result=client.query(
            "select user_id,user_name,user_email,user_password,ban_login,ban_judge
             from users where user_id=$1",&[&user_id]);
        match result{
            Ok(rows)=>{
                match(rows.len()){
                    1=>{
                        let row=rows.get(0).unwrap();
                        User::new(
                            row.get(0),
                            row.get(1),
                            row.get(2),
                            row.get(3),
                            row.get(4),
                            row.get(5),
                        )
                    },
                    _=>Err(()),
                }
            }
            Err(_)=>Err(()),
        }
    }
}
    

