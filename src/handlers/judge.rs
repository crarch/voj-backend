use actix_web::{web,HttpRequest,HttpResponse,post,Error,get};
use mongodb::bson::doc;

use crate::MongoDB;

use crate::models::get_question_update_by_id;
use crate::models::create_new_record;
use crate::models::queue_add_job;

use crate::models::CodeJson;
use crate::models::UserId;

#[post("/judge")]
pub async fn judge(
    mongo:MongoDB,
    code_json:web::Json<CodeJson>,
    user_id:UserId
)->Result<HttpResponse,Error>{
    
    let question_id=code_json.question_id;
    let code=&code_json.code;
    
    if let Ok(update)=get_question_update_by_id(mongo.clone(),code_json.question_id){
        if let Ok(result)=create_new_record(mongo.clone(),user_id.user_id,question_id,code){
            let object_id=result.get_str("$oid").unwrap();
            
            queue_add_job(
                mongo,
                object_id,
                question_id,
                update,
                code
            ).unwrap();
            
            let result=doc!{
                "_id":result
            };
            
            return Ok(HttpResponse::Ok().json(result));
        }
    }
    
    Ok(HttpResponse::NotFound().finish())
}

use crate::models::get_record_by_object_id;

#[get("/judge/record/{object_id}")]
pub async fn get_record(
    mongo:MongoDB,
    path: web::Path<(String,)>,
    user_id:UserId
)->Result<HttpResponse,Error>{
    
    if let Ok(result)=get_record_by_object_id(mongo,&path.into_inner().0,user_id.user_id){
        return Ok(HttpResponse::Ok().json(result));
    }
    
    Ok(HttpResponse::NotFound().finish())
}
