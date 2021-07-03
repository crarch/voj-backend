use actix_web::{middleware::Logger,App,HttpServer,HttpResponse};
use actix_web::dev::Service;
use actix_web::{HttpMessage};



use crate::utils::jwt::validate_jwt;
use crate::database::get_database_pool;

use crate::routes::routing;
use crate::env::get_env;
use crate::models::UserId;

pub async fn server()->std::io::Result<()>{
    
    let database_pool=get_database_pool();
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let listen:String=get_env("LISTEN_IP")+":"+&(get_env("LISTEN_PORT"));
    
    
    HttpServer::new(move||{
        App::new()
            .wrap_fn(|req,srv|{
                let mut authorized=false;
                if let Some(authorization)=req.headers().get("Authorization"){
                    if let Ok(token)=authorization.to_str(){
                        if let Ok(token_data)=validate_jwt(token){
                            let user_id=token_data.claims.get_user_id();
                            let user_id=UserId{user_id:user_id};
                            req.extensions_mut().insert(user_id);
                            
                            authorized=true;
                        }
                    }
                }
                
                match(authorized){
                    true=>srv.call(req),
                    false=>{
                        if(*&req.path().starts_with("/session")||*&req.path().starts_with("/version")){
                            srv.call(req)
                        }else{
                            Box::pin(async move {
                                Ok(req.into_response(
                                    HttpResponse::Unauthorized()
                                        .finish()
                                        .into_body(),
                                ))
                            })
                        }      
                    }
                }
            })
                        
            .data(database_pool.clone())
                
            .configure(routing)
            .wrap(Logger::new("%a \"%r\" %s"))
    })
        .bind(&listen[..])?
        .run()
        .await
}


