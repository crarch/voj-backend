use actix_web::{App,HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_web::{http,web::Data};
use actix::Addr;

use crate::routes::routing;
use crate::env::get_env;
use crate::middleware;
use crate::database::get_db;




use actix_cors::Cors;
use actix::Actor;
use actix_web::middleware::Logger;

pub type Queue=Data<Addr<crate::actors::Queue>>;

pub async fn server()->std::io::Result<()>{
    
    env_logger::init();

    let listen:String=get_env("LISTEN_IP")+":"+&(get_env("LISTEN_PORT"));
    let ssl_on=get_env("SSL_ON");
    
        
    let mongo=get_db().await;
    
    let actor_queue=crate::actors::Queue::new(mongo.clone()).start();
    
    let httpserver=HttpServer::new(move||{
        let cors_origin=get_env("CORS_ORIGIN");
        
        let cors = Cors::default()
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
              
        let cors=
            if(get_env("ALLOW_ANY_ORIGIN")=="true"){
                cors.allow_any_origin()
            }else{
                cors.allowed_origin(&cors_origin)
            };
            
        
        App::new()
            .wrap(middleware::Auth)
            .wrap(cors)
            .configure(routing)
            .app_data(Data::new(mongo.clone()))
            .app_data(Data::new(actor_queue.clone()))
            .wrap(Logger::default())
    })
        .keep_alive(75);
    
    let httpserver=
        if(ssl_on=="true"){
            
            let ssl_key=get_env("SSL_KEY");
            let ssl_cert=get_env("SSL_CERT");
            
            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
            .set_private_key_file(&ssl_key, SslFiletype::PEM)
            .unwrap();
            builder.set_certificate_chain_file(&ssl_cert).unwrap();
            httpserver.bind_openssl(&listen,builder)?
            
        }else{
            httpserver.bind(&listen)?
        };
        
    httpserver.run().await
}


