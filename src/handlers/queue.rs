use actix_web::{web,HttpRequest,HttpResponse,post,Error,get};

use crate::MongoDB;


use crate::models::query_first_job;
use crate::models::delete_job_by_id;
use crate::models::update_judge_result;
use crate::models::queue::JudgeResultJson;
use crate::models::pass::add_pass_by_id;


use crate::utils::env::get_env;

#[get("/queue")]
pub async fn get_first_job(
    mongo:MongoDB,
    req:HttpRequest,
)->Result<HttpResponse,Error>{
    if let Some(authorization)=req.headers().get("Authorization"){
        if let Ok(key)=authorization.to_str(){
            if(key==get_env("JUDGER_KEY")){
                let result=match query_first_job(mongo).await{
                    Ok(result)=>HttpResponse::Ok().json(result),
                    Err(_)=>HttpResponse::Ok().body(""),
                };
                return Ok(result);
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
                let _result=update_judge_result(
                    mongo.clone(),
                    &judge_result._id,
                    judge_result.success,
                    &judge_result.test_bench
                ).await.unwrap();
                
                if(judge_result.success){
                    let _result=add_pass_by_id(
                        mongo.clone(),
                        judge_result.user_id,
                        judge_result.question_id
                    ).await.unwrap();    
                }
                    
                let _result=delete_job_by_id(
                    mongo,
                    &judge_result._id
                ).await.unwrap();
                
                
                return Ok(HttpResponse::Ok().finish());
                            
            }
        }
    }
    Ok(HttpResponse::Unauthorized()
        .finish()
    )
}

