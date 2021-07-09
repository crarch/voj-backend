use actix_web::{App,HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use actix_web::{web::Data};


use crate::database::get_mongo_database;

use crate::routes::routing;
use crate::env::get_env;
use crate::models::cron;

use crate::middleware;


pub async fn server()->std::io::Result<()>{
    
    let mongodb=get_mongo_database();
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let listen:String=get_env("LISTEN_IP")+":"+&(get_env("LISTEN_PORT"));
    
    let ssl_key=get_env("SSL_KEY");
    let ssl_cert=get_env("SSL_CERT");
    
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&ssl_key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(&ssl_cert).unwrap();
        
    cron(Data::new(mongodb.clone()));
    
    
    HttpServer::new(move||{
        App::new()
            .wrap(middleware::Auth)
            .configure(routing)
            .app_data(Data::new(mongodb.clone()))
            // .wrap(Logger::new("%a \"%r\" %s"))
    })
        .bind_openssl(&listen,builder)?
        .run()
        .await
}


