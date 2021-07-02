use std::thread;
use r2d2;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager<NoTls>>;

pub fn get_database_pool()->Pool{
    
    dotenv::dotenv().ok();
    
    let pg_host = std::env::var("POSTGRES_HOST").expect("missing environment variable POSTGRES_HOST");
    let pg_user = std::env::var("POSTGRES_USER").expect("missing environment variable POSTGRES_USER");
    let pg_password = std::env::var("POSTGRES_PASSWORD").expect("missing environment variable POSTGRES_PASSWORD");
    let pg_dbname = std::env::var("POSTGRES_DBNAME").unwrap_or("postgres".to_string());
    let pg_port = std::env::var("POSTGRES_PORT").unwrap_or("5432".to_string());

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
    
