use std::sync::Arc;

use crate::utils::serializers::serialize_dt;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TodoTask {
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

#[derive(Deserialize)]
pub struct GetTasksPayload {
    pub completed: Option<bool>,
}

pub async fn get_all_tasks(
    State(db): State<Arc<PgPool>>,
) -> Result<(StatusCode, Json<Vec<TodoTask>>), (StatusCode, String)> {
    let q = "SELECT * FROM tasks";
    let query = sqlx::query_as::<_, TodoTask>(q);
    let tasks = query.fetch_all(&*db).await;

    match tasks {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_pending_tasks(
    State(db): State<Arc<PgPool>>,
) -> Result<(StatusCode, Json<Vec<TodoTask>>), (StatusCode, String)> {
    let q = "SELECT * FROM tasks WHERE completed_at IS NULL";
    let query = sqlx::query_as::<_, TodoTask>(q);
    let tasks = query.fetch_all(&*db).await;

    match tasks {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_completed_tasks(
    State(db): State<Arc<PgPool>>,
) -> Result<(StatusCode, Json<Vec<TodoTask>>), (StatusCode, String)> {
    let q = "SELECT * FROM tasks WHERE completed_at IS NOT NULL";
    let query = sqlx::query_as::<_, TodoTask>(q);
    let tasks = query.fetch_all(&*db).await;

    match tasks {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
