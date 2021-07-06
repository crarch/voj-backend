

use actix_web::web;

use crate::handlers::{
    get_version,
};

use crate::handlers::session::*;
use crate::handlers::profile::*;
use crate::handlers::question::*;
use crate::handlers::judge::*;
use crate::handlers::queue::*;

pub fn routing(cfg:&mut web::ServiceConfig){
    cfg
        .service(get_question)
        .service(return_judge_result)
        .service(get_first_job)
        .service(get_record)
        .service(judge)
        .service(get_jwt_token)
        .service(get_pass)
        .service(refresh_jwt_token)
        .service(get_version);
        
}
            


