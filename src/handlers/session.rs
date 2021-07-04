use actix_web::{web,HttpRequest,HttpResponse,post,Error,get};
use anyhow::Result;

use crate::MongoDB;

use crate::utils::auth::UserAuthJson;



use crate::utils::jwt::{validate_jwt,sign_jwt};

use crate::utils::auth::auth_user;

#[post("/session")]
pub async fn get_jwt_token(
    mongo:MongoDB,
    user_auth_json:web::Json<UserAuthJson>
)->Result<HttpResponse,Error>{
    if let Ok(user_id)=auth_user(
        mongo,
        &user_auth_json.user_email,
        &user_auth_json.user_password
    ){
        let jwt_token=sign_jwt(user_id)?;
        
        let body=format!("{{\"Authorization\":\"{}\"}}",jwt_token);
        
        let result=HttpResponse::Ok()
            .content_type("application/json")
            .body(body);
        Ok(result)
    }else{
        Ok(HttpResponse::Unauthorized()
            .finish()
            .into_body()
        )
    }
}

#[get("/session")]
pub async fn refresh_jwt_token(
    req:HttpRequest
)->Result<HttpResponse,Error>{
    if let Some(authorization)=req.headers().get("Authorization"){
        if let Ok(token)=authorization.to_str(){
            if let Ok(token_data)=validate_jwt(token){
                let user_id=token_data.claims.get_user_id();
                
                let jwt_token=sign_jwt(user_id)?;
                
                let body=format!("{{\"Authorization\":\"{}\"}}",jwt_token);
                
                let result=HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body);
                return Ok(result);
                
            }
        }
    }
    Ok(HttpResponse::Unauthorized()
        .finish()
        .into_body()
    )
}



