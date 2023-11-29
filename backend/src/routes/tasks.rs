use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::controllers::tasks::*;

pub fn tasks_router() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/", get(get_tasks).post(create_task))
        .route("/:id", get(get_task).delete(delete_task))
        .route("/:id/complete", post(complete_task))
}
