use axum::{
    routing::{get, post},
    Router,
};

use crate::controllers::tasks::*;

pub fn tasks_router() -> Router {
    Router::new()
        .route("/", get(get_tasks).post(create_task))
        .route("/:id", get(get_task).delete(delete_task))
        .route("/:id/complete", post(complete_task))
}
