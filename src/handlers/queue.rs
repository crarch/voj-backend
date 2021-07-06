use actix_web::{web,HttpRequest,HttpResponse,post,Error,get};

use crate::MongoDB;



use crate::models::queue_get_first_job;
use crate::models::queue_delete_job_by_id;
use crate::models::queue_update_judge_result;
use crate::models::queue::JudgeResultJson;




use crate::utils::env::get_env;

#[get("/queue")]
pub async fn get_first_job(
    mongo:MongoDB,
    req:HttpRequest,
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



#[post("/queue")]
pub async fn return_judge_result(
    mongo:MongoDB,
    req:HttpRequest,
    judge_result:web::Json<JudgeResultJson>
)->Result<HttpResponse,Error>{
    
    if let Some(authorization)=req.headers().get("Authorization"){
        if let Ok(key)=authorization.to_str(){
            if(key==get_env("JUDGER_KEY")){
                if let Ok(_result)=queue_update_judge_result(
                    mongo.clone(),
                    &judge_result.object_id,
                    judge_result.success,
                    &judge_result.test_bench
                ){
                    if let Ok(_result)=queue_delete_job_by_id(mongo,&judge_result.object_id){
                        return Ok(HttpResponse::Ok().finish());
                    }                
                }
                return Ok(HttpResponse::InternalServerError().finish());
                            
            }
        }
    }
    Ok(HttpResponse::Unauthorized()
        .finish()
    )
}

