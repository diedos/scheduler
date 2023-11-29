use std::sync::Arc;

use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use serde::Deserialize;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::models::tasks::*;

pub async fn get_tasks(
    State(db): State<Arc<PgPool>>,
    Json(payload): Json<GetTasksPayload>,
) -> Result<(StatusCode, Json<Vec<TodoTask>>), (StatusCode, String)> {
    if let Some(value) = &payload.completed {
        match value {
            true => return get_completed_tasks(State(db)).await,
            false => return get_pending_tasks(State(db)).await,
        }
    } else {
        return get_all_tasks(State(db)).await;
    };
}

pub async fn delete_task(
    State(db): State<Arc<PgPool>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<TodoTask>), (StatusCode, String)> {
    let query = "DELETE FROM tasks WHERE id = $1 RETURNING *";
    let result = sqlx::query_as::<_, TodoTask>(query)
        .bind(id)
        .fetch_one(&*db)
        .await;

    match result {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn complete_task(
    State(db): State<Arc<PgPool>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<TodoTask>), (StatusCode, String)> {
    let query = "UPDATE tasks SET completed_at = NOW() WHERE id = $1";
    let _result = sqlx::query_as::<_, TodoTask>(query)
        .bind(id)
        .fetch_one(&*db)
        .await;

    let query2 = "SELECT * FROM tasks WHERE completed_at IS NULL ORDER BY deadline_at ASC, created_at ASC LIMIT 1";
    let result2 = sqlx::query_as::<_, TodoTask>(query2)
        .bind(id)
        .fetch_one(&*db)
        .await;

    match result2 {
        Ok(result2) => Ok((StatusCode::OK, Json(result2))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_task(
    State(db): State<Arc<PgPool>>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<TodoTask>), (StatusCode, String)> {
    let query = "SELECT * FROM tasks WHERE id = $1";
    let result = sqlx::query_as::<_, TodoTask>(query)
        .bind(id)
        .fetch_one(&*db)
        .await;

    match result {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskPayload {
    title: String,
    content: Option<String>,
    deadline_at: Option<String>,
    estimate: Option<i16>,
}

pub async fn create_task(
    State(db): State<Arc<PgPool>>,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<(StatusCode, Json<TodoTask>), (StatusCode, String)> {
    if payload.title.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Title is required".to_string()));
    }

    let deadline_at = if let Some(value) = &payload.deadline_at {
        NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M").ok()
    } else {
        None
    };

    let query = "INSERT INTO tasks (title, content, deadline_at, estimate) VALUES ($1, $2, $3, $4) RETURNING *";
    let result = sqlx::query_as::<_, TodoTask>(query)
        .bind(payload.title)
        .bind(payload.content)
        .bind(deadline_at)
        .bind(payload.estimate)
        .fetch_one(&*db)
        .await;

    match result {
        Ok(result) => Ok((StatusCode::CREATED, Json(result))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
