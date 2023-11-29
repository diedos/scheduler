use dotenv;
use std::env;
pub struct Config {
    pub root_url: String,
    pub db_host: String,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
}

impl Config {
    pub fn init() -> Config {
        dotenv::dotenv().ok();
        // General
        let root_url = env::var("ROOT_URL").expect("ROOT_URL environment variable must be set");

        // Database
        let db_host = env::var("PG_HOST").expect("PG_HOST environment variable must be set");
        let db_name = env::var("PG_DBNAME").expect("PG_DBNAME environment variable must be set");
        let db_user = env::var("PG_USER").expect("PG_USER environment variable must be set");
        let db_pass =
            env::var("PG_PASSWORD").expect("PG_PASSWORD environment variable must be set");

        // Authentication
        let google_oauth_client_id = env::var("GOOGLE_OAUTH_CLIENT_ID")
            .expect("GOOGLE_OAUTH_CLIENT_ID environment variable must be set");
        let google_oauth_client_secret = env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .expect("GOOGLE_OAUTH_CLIENT_SECRET environment variable must be set");

        Config {
            root_url,
            db_host,
            db_name,
            db_user,
            db_pass,
            google_oauth_client_id,
            google_oauth_client_secret,
        }
    }
}
