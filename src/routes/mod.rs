use actix_web::web;

use crate::handlers::{
    get_version,
};

use crate::handlers::session::*;
use crate::handlers::profile::*;
use crate::handlers::testbench::*;
use crate::handlers::judge::*;
use crate::handlers::queue::*;

pub fn routing(cfg:&mut web::ServiceConfig){
    cfg
        .service(get_record_list_by_question)
        .service(get_record_paging)
        .service(get_testbench)
        .service(get_record_list)
        .service(return_judge_result)
        .service(get_first_job)
        .service(get_record)
        .service(judge)
        .service(get_jwt_token)
        .service(get_pass)
        .service(refresh_jwt_token)
        .service(get_version);
        
}
            


