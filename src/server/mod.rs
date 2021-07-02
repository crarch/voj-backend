use actix_web::{middleware::Logger,App,HttpServer,web};

use crate::database::get_database_pool;

use crate::routes::routing;

pub async fn server()->std::io::Result<()>{
    
    let database_pool=get_database_pool();
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    HttpServer::new(move||{
        App::new()
            .data(database_pool.clone())
            .configure(routing)
            .wrap(Logger::new("%a \"%r\" %s"))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

