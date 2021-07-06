use actix_web::{web,middleware::Logger,App,HttpServer};

use actix_web::{web::Data};


use crate::database::get_mongo_database;

use crate::routes::routing;
use crate::env::get_env;

use crate::middleware;


pub async fn server()->std::io::Result<()>{
    
    let mongodb=get_mongo_database();
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let listen:String=get_env("LISTEN_IP")+":"+&(get_env("LISTEN_PORT"));
    
    HttpServer::new(move||{
        App::new()
            .service(web::scope("/profile").wrap(middleware::Auth))
            .service(web::scope("/judge").wrap(middleware::Auth))
            .configure(routing)
            .app_data(Data::new(mongodb.clone()))
            .wrap(Logger::new("%a \"%r\" %s"))
    })
        .bind(&listen)?
        .run()
        .await
}


