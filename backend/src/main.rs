use axum::routing::post;
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod controllers;
mod models;
mod utils;

use crate::controllers::tasks::*;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_host = env::var("PG_HOST").expect("PG_HOST environment variable should be set");
    let db_name = env::var("PG_DBNAME").expect("PG_DBNAME environment variable should be set");
    let db_user = env::var("PG_USER").expect("PG_USER environment variable should be set");
    let db_pass = env::var("PG_PASSWORD").expect("PG_PASSWORD environment variable should be set");

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&format!(
            "postgres://{}:{}@{}/{}",
            db_user, db_pass, db_host, db_name
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
