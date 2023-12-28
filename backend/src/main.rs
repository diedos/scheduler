use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

mod config;
mod controllers;
mod middlewares;
mod models;
mod routes;
mod utils;

use crate::routes::root::root_router;

lazy_static::lazy_static! {
    pub static ref CONFIG: config::Config = config::Config::init();
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&format!(
            "postgres://{}:{}@{}/{}",
            &CONFIG.db_user, &CONFIG.db_pass, &CONFIG.db_host, &CONFIG.db_name
        ))
        .await
        .expect("Database connection failed");

    sqlx::migrate!().run(&db_pool).await?;

    let db = Arc::new(db_pool);

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());

    let app = Router::new().merge(root_router(db.clone())).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
