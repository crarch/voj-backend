use actix_web::{web,HttpResponse,post,Error,get};
use mongodb::bson::doc;

use crate::MongoDB;

use crate::models::query_testbench_update_by_id;
use crate::models::create_new_record;
use crate::models::add_job;
use crate::models::query_record_list_by_page;
use crate::models::query_record_list_by_page_and_question;
use crate::models::query_record_by_object_id;

use crate::models::CodeJson;
use crate::models::UserId;

#[post("/judge")]
pub async fn judge(
    mongo:MongoDB,
    code_json:web::Json<CodeJson>,
    user_id:UserId
)->Result<HttpResponse,Error>{
    let user_id=user_id.user_id;
    
    let question_id=code_json.question_id;
    let code=&code_json.code;
    
    if let Ok(update)=query_testbench_update_by_id(mongo.clone(),code_json.question_id).await{
        if let Ok(result)=create_new_record(mongo.clone(),user_id,question_id,code).await{
            let object_id=result.get_str("$oid").unwrap();
            
            add_job(
                mongo,
                object_id,
                question_id,
                update,
                user_id,
                code
            ).await.unwrap();
            
            let result=doc!{
                "_id":result
            };
            
            return Ok(HttpResponse::Ok().json(result));
        }
    }
    
    Ok(HttpResponse::NotFound().finish())
}


#[get("/judge/record/{object_id}")]
pub async fn get_record(
    mongo:MongoDB,
    path: web::Path<(String,)>,
    user_id:UserId
)->Result<HttpResponse,Error>{
    
    if let Ok(result)=query_record_by_object_id(mongo,&path.into_inner().0,user_id.user_id).await{
        return Ok(HttpResponse::Ok().json(result));
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

