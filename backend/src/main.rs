use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let origins = ["http://localhost:5173".parse().unwrap()];
    let cors = CorsLayer::new().allow_origin(origins);

    let app = Router::new()
        .route("/", get(root))
        .route("/tasks", get(all_tasks))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Serialize)]
struct TodoTask {
    id: u32,
    created_at: String,
    modified_at: String,
    title: String,
    content: String,
    completed: bool,
}

async fn all_tasks() -> impl IntoResponse {
    let test_todo = vec![
        TodoTask {
            id: 12,
            created_at: "1999-01-08 04:05:06".to_string(),
            modified_at: "2023-02-11 12:50:10".to_string(),
            title: "First task".to_string(),
            content: "Create a Hello World program yay".to_string(),
            completed: false,
        },
        TodoTask {
            id: 13,
            created_at: "2021-02-12 12:05:06".to_string(),
            modified_at: "2023-02-11 12:50:10".to_string(),
            title: "Second task".to_string(),
            content: "Complete the task".to_string(),
            completed: true,
        },
        TodoTask {
            id: 14,
            created_at: "2022-05-08 14:05:06".to_string(),
            modified_at: "2023-02-11 12:50:10".to_string(),
            title: "Third task".to_string(),
            content: "I don't know".to_string(),
            completed: false,
        },
    ];

    (StatusCode::OK, Json(test_todo))
}
