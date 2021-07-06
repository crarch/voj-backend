use actix_web::{web,HttpRequest,HttpResponse,post,Error,get};

use crate::MongoDB;

use crate::models::get_question_update_by_id;

use crate::models::CodeJson;

#[post("/judge")]
pub async fn judge(
    mongo:MongoDB,
    code_json:web::Json<CodeJson>
)->Result<HttpResponse,Error>{
    
    if let Ok(_result)=get_question_update_by_id(mongo,code_json._id){
        return Ok(HttpResponse::Ok().body(""));
    }
    
    Ok(HttpResponse::NotFound().finish())
}



