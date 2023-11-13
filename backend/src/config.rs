use std::env;

pub struct Config {
    pub db_host: String,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
}

impl Config {
    pub fn init() -> Config {
        let db_host = env::var("PG_HOST").expect("PG_HOST environment variable must be set");
        let db_name = env::var("PG_DBNAME").expect("PG_DBNAME environment variable must be set");
        let db_user = env::var("PG_USER").expect("PG_USER environment variable must be set");
        let db_pass =
            env::var("PG_PASSWORD").expect("PG_PASSWORD environment variable must be set");

        Config {
            db_host,
            db_name,
            db_user,
            db_pass,
        }
    }
}
