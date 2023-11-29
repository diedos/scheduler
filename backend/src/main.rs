use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod config;
mod controllers;
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

    //let origins = ["http://localhost:5173".parse().unwrap()];
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .merge(root_router())
        .with_state(db)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
