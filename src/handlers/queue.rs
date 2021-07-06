use actix_web::{web,HttpRequest,HttpResponse,post,Error,get};

use crate::MongoDB;

use crate::models::queue_get_first_job;

use crate::models::CodeJson;
use crate::models::UserId;

use crate::utils::env::get_env;

#[get("/queue")]
pub async fn get_first_job(
    mongo:MongoDB,
    req:HttpRequest
)->Result<HttpResponse,Error>{
    if let Some(authorization)=req.headers().get("Authorization"){
        if let Ok(key)=authorization.to_str(){
            if(key==get_env("JUDGER_KEY")){
                if let Some(judger_id)=req.headers().get("JudgerId"){
                    if let Ok(judger_id)=judger_id.to_str(){
                        if let Ok(judger_id)=judger_id.parse::<u32>(){
                    
                            let result=match queue_get_first_job(mongo,judger_id){
                                Ok(result)=>HttpResponse::Ok().json(result),
                                Err(_)=>HttpResponse::Ok().body(""),
                            };
                            return Ok(result);
                        }
                    }
                }
            }
        }
    }
    Ok(HttpResponse::Unauthorized()
        .finish()
    )
}

