use actix_web::{middleware::Logger,App,HttpServer,web};

use crate::database::get_database_pool;

use crate::routes::routing;
use crate::env::get_env;

pub async fn server()->std::io::Result<()>{
    
    let database_pool=get_database_pool();
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let listen:String=get_env("LISTEN_IP")+":"+&(get_env("LISTEN_PORT"));
    
    HttpServer::new(move||{
        App::new()
            .data(database_pool.clone())
            .configure(routing)
            .wrap(Logger::new("%a \"%r\" %s"))
    })
        .bind(&listen[..])?
        .run()
        .await
}

