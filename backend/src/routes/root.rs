use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use sqlx::PgPool;

use super::{auth::auth_router, tasks::tasks_router};

pub fn root_router(db: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello" }))
        .nest("/auth", auth_router())
        .nest("/tasks", tasks_router())
        .layer(Extension(db))
}
