use serde::{Deserialize,Serialize};

use crate::Pool;


#[derive(Debug,Serialize,Deserialize)]
pub struct User{
    pub user_id:u32,
    pub user_name:String,
    pub user_email:String,
    pub user_password:String,
    pub ban_login:bool,
    pub ban_judge:bool,
}

impl User{
    pub fn new(
        user_id:u32,
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
        pg_pool:Pool,
        user_id:u32,
    )->Result<User,()>{
        let mut client=pg_pool.get().unwrap();
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
    
pub fn get_user_password_by_email(
    pg_pool:Pool,
    user_email:&str,
)->Result<(u32,String),()>{
    let mut client=pg_pool.get().unwrap();
    let result=client.query(
        "select user_id,user_password
         from users where user_email=$1",&[&user_email.to_string()]);
    match result{
        Ok(rows)=>{
            match(rows.len()){
                1=>{
                    let row=rows.get(0).unwrap();
                    let user_id:i32=row.get(0);
                    let user_id:u32=user_id as u32;
                    let user_password:String=row.get(1);
                    Ok((user_id,user_password))
                    
                },
                _=>Err(()),
            }
        }
        Err(_)=>Err(()),
    }
}


