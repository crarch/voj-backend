use actix_web::{App,HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use actix_web::{http,web::Data};

use crate::routes::routing;
use crate::env::get_env;
use crate::models::cron;

use crate::middleware;
use crate::database::get_db;

use std::sync::Mutex;

use bson::oid::ObjectId;
pub type Queue=Data<Mutex<VecDeque<ObjectId>>>;
use std::collections::VecDeque;

use actix_cors::Cors;

pub async fn server()->std::io::Result<()>{
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let listen:String=get_env("LISTEN_IP")+":"+&(get_env("LISTEN_PORT"));
    
    let ssl_key=get_env("SSL_KEY");
    let ssl_cert=get_env("SSL_CERT");
    
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&ssl_key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(&ssl_cert).unwrap();
        
    let mongo=get_db().await;
    
    let queue:Queue=Data::new(Mutex::new(VecDeque::new()));

        
    cron(Data::new(mongo.clone())).await;
    
    HttpServer::new(move||{
        let cors_origin=get_env("CORS_ORIGIN");
        
        let cors = Cors::default()
              .allowed_origin(&cors_origin)
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        
        App::new()
            .wrap(cors)
            .wrap(middleware::Auth)
            .configure(routing)
            .app_data(Data::new(mongo.clone()))
            .app_data(queue.clone())
            .wrap(Logger::default())
    })
        .keep_alive(75)
        .bind_openssl(&listen,builder)?
        .run()
        .await
}


use actix_web::middleware::Logger;