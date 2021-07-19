pub mod session;
pub mod profile;
pub mod testbench;
pub mod judge;

use crate::env;

use actix_web::{web,HttpResponse,Error,get,HttpRequest};
use actix_web::web::Data;
use anyhow::Result;

use actix_web_actors::ws;
use crate::actors::{JudgerWs,Queue};
use actix::Addr;


use crate::utils::env::get_env;




#[get("/version")]
pub async fn get_version()->Result<HttpResponse,Error>{
    Ok(HttpResponse::Ok().body(format!("{{\"version\":\"{}\"}}",env::VERSION)))
}


#[get("/websocket")]
pub async fn get_websocket(
    req:HttpRequest,
    stream:web::Payload,
    queue:Data<Addr<Queue>>,
)->Result<HttpResponse,Error>{
    if let Some(authorization)=req.headers().get("Authorization"){
        if let Ok(key)=authorization.to_str(){
            if(key==get_env("JUDGER_KEY")){
                let ws=JudgerWs::new(
                    queue.get_ref().clone(),
                );
                
                let resp=ws::start(ws,&req,stream);
                return resp;
            }
        }
    }
    Ok(HttpResponse::Unauthorized()
        .finish()
    )
}
