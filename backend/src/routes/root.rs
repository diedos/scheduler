use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::PgPool;

use super::{auth::auth_router, tasks::tasks_router};

pub fn root_router() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/", get(|| async { "Hello" }))
        .nest("/auth", auth_router())
        .nest("/tasks", tasks_router())
}
