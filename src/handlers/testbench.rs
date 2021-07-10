use actix_web::{web,HttpResponse,Error,get};

use crate::MongoDB;

use crate::models::query_testbench_by_id;


#[get("/testbench/{id}")]
pub async fn get_testbench(
    mongo:MongoDB,
    path: web::Path<(u32,)>
)->Result<HttpResponse,Error>{
    
    if let Ok(result)=query_testbench_by_id(mongo,path.into_inner().0).await{
        return Ok(HttpResponse::Ok().json(result));
    }
    
    Ok(HttpResponse::NotFound().finish())
}



