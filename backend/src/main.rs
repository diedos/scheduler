use axum::routing::post;
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod config;
mod controllers;
mod models;
mod utils;

use crate::controllers::tasks::*;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let config = config::Config::init();
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&format!(
            "postgres://{}:{}@{}/{}",
            config.db_user, config.db_pass, config.db_host, config.db_name
        ))
        .await
        .expect("Database connection failed");

    sqlx::migrate!().run(&db).await?;

    //let origins = ["http://localhost:5173".parse().unwrap()];
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/:id", get(get_task).delete(delete_task))
        .route("/tasks/:id/complete", post(complete_task))
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

async fn root() -> &'static str {
    "Hello, World!"
}
