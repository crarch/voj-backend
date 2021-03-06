use actix_web::{web,HttpResponse,post,Error,get};
use mongodb::bson::doc;
use bson::oid::ObjectId;
use serde::{Deserialize,Serialize};
use crate::actors::Job;

use crate::MongoDB;
use crate::Queue;
use crate::models::query_record_list_by_page;
use crate::models::query_record_list_by_page_and_question;
use crate::models::query_record_by_object_id;
use crate::models::query_record_paging;
use crate::models::UserId;
use crate::actors::push_job;
use crate::utils::time::get_unix_timestamp;


#[post("/judge")]
pub async fn judge(
    mut code_json:web::Json<CodeJson>,
    user_id:UserId,
    queue:Queue
)->Result<HttpResponse,Error>{
    let user_id=user_id.user_id;
    let question_id=code_json.question_id;
    let code=std::mem::take(&mut code_json.code);
    
    let object_id=ObjectId::new();
    
    let job=Job{
        _id:object_id.clone(),
        success:false,
        test_bench:doc!{},
        question_id:question_id,
        user_id:user_id,
        code:code,
        submit_time:get_unix_timestamp(),
    };
    
    
    push_job(queue,job).await;

    let result=doc!{
        "_id":object_id.clone()
    };
    
    return Ok(HttpResponse::Ok().json(result));
    
}
#[get("/judge/record/{object_id}")]
pub async fn get_record(
    mongo:MongoDB,
    path: web::Path<(String,)>,
    user_id:UserId
)->Result<HttpResponse,Error>{
    
    if let Ok(object_id)=ObjectId::parse_str(&path.into_inner().0){
        if let Ok(result)=query_record_by_object_id(mongo,object_id,user_id.user_id).await{
            return Ok(HttpResponse::Ok().json(result));
        }
    }
    
    Ok(HttpResponse::NotFound().finish())
}

#[get("/judge/record/page/{page}")]
pub async fn get_record_list(
    mongo:MongoDB,
    path: web::Path<(u64,)>,
    user_id:UserId
)->Result<HttpResponse,Error>{

    if let Ok(result)=query_record_list_by_page(mongo,path.into_inner().0,user_id.user_id).await{
        return Ok(HttpResponse::Ok().json(result));
    }

    Ok(HttpResponse::NotFound().finish())
}

#[get("/judge/record/page")]
pub async fn get_record_paging(
    mongo:MongoDB,
    user_id:UserId
)->Result<HttpResponse,Error>{

    if let Ok(result)=query_record_paging(mongo,user_id.user_id).await{
        return Ok(HttpResponse::Ok().json(result));
    }

    Ok(HttpResponse::NotFound().finish())
}

#[get("/judge/record/question/{id}/page/{page}")]
pub async fn get_record_list_by_question(
    mongo:MongoDB,
    path: web::Path<(u32,u64)>,
    user_id:UserId
)->Result<HttpResponse,Error>{
    let path=path.into_inner();
    if let Ok(result)=query_record_list_by_page_and_question(
        mongo,
        path.0,
        path.1,
        user_id.user_id
    ).await{
        return Ok(HttpResponse::Ok().json(result));
    }

    Ok(HttpResponse::NotFound().finish())
}


#[derive(Debug,Serialize,Deserialize)]
pub struct CodeJson{
    pub question_id:u32,
    pub code:String
}
