

use actix_web::web;

use crate::handlers::{
    get_version,
};

use crate::handlers::session::*;

pub fn routing(cfg:&mut web::ServiceConfig){
    cfg
        .service(get_jwt_token)
        .service(get_version);
}
            


