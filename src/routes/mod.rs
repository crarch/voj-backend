use actix_web::web;

use crate::handlers::{
    get_version,
};

use crate::handlers::session::*;
use crate::handlers::profile::*;
use crate::handlers::judge::*;


use crate::handlers::get_websocket;

pub fn routing(cfg:&mut web::ServiceConfig){
    cfg
        .service(get_websocket)
        .service(get_record_list_by_question)
        .service(get_record_paging)
        .service(get_record_list)
        .service(get_record)
        .service(judge)
        .service(get_jwt_token)
        .service(get_pass)
        .service(refresh_jwt_token)
        .service(get_user_profile)
        .service(get_version);
        
}
            


