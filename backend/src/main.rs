use axum::extract::{Json, State};
use axum::{http::StatusCode, routing::get, Router};
use serde::{Deserialize, Serialize, Serializer};
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod errors;
mod server;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().expect("Failed to read .env file");

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable should be set");

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Database connection failed");

    sqlx::migrate!().run(&db).await?;

    let origins = ["http://localhost:5173".parse().unwrap()];
    let cors = CorsLayer::new().allow_origin(origins);

    let app = Router::new()
        .route("/", get(root))
        .route("/tasks", get(all_tasks).post(create_task))
        .with_state(db)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
struct TodoTask {
    pub id: i32,
    #[serde(serialize_with = "serialize_dt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(serialize_with = "serialize_dt")]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(serialize_with = "serialize_dt")]
    pub completed_at: Option<NaiveDateTime>,
    #[serde(serialize_with = "serialize_dt")]
    pub deadline_at: Option<NaiveDateTime>,
    pub estimate: Option<i16>,
    pub title: String,
    pub content: Option<String>,
}

#[axum_macros::debug_handler]
async fn all_tasks(
    State(db): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<TodoTask>>), (StatusCode, String)> {
    let q = "SELECT * FROM tasks";
    let query = sqlx::query_as::<_, TodoTask>(q);
    let tasks = query.fetch_all(&db).await;

    match tasks {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateTaskPayload {
    title: String,
    content: Option<String>,
    deadline_at: Option<String>,
    estimate: Option<i16>,
}

async fn create_task(
    State(db): State<PgPool>,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<(StatusCode, Json<TodoTask>), (StatusCode, String)> {
    let deadline_at = if let Some(value) = &payload.deadline_at {
        NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S").ok()
    } else {
        None
    };

    let query = "INSERT INTO tasks (title, content, deadline_at, estimate) VALUES ($1, $2, $3, $4) RETURNING *";
    let result = sqlx::query_as::<_, TodoTask>(query)
        .bind(payload.title)
        .bind(payload.content)
        .bind(deadline_at)
        .bind(payload.estimate)
        .fetch_one(&db)
        .await;

    match result {
        Ok(result) => Ok((StatusCode::CREATED, Json(result))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub fn serialize_dt<S>(dt: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(dt) = dt {
        dt.format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}
