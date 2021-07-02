use std::thread;
use r2d2;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager<NoTls>>;

use crate::env::get_env;

pub fn get_database_pool()->Pool{
    
    dotenv::dotenv().ok();
    
    let pg_host = get_env("POSTGRES_HOST");
    let pg_user = get_env("POSTGRES_USER");
    let pg_password = get_env("POSTGRES_PASSWORD");
    let pg_dbname = get_env("POSTGRES_DBNAME");
    let pg_port = get_env("POSTGRES_PORT");

    let database_info = format!(
        "host={} port={} user={} password={} dbname={}",
        pg_host, pg_port, pg_user, pg_password, pg_dbname
    );
    
    let manager=PostgresConnectionManager::new(
        database_info.parse().unwrap(),
        NoTls,
    );
    
    let pool=r2d2::Pool::new(manager).unwrap();
    
    pool
}
    
