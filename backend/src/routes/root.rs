use std::sync::Arc;

use axum::{middleware, routing::get, Extension, Router};
use sqlx::PgPool;

use crate::middlewares::auth::auth;

use super::{auth::auth_router, tasks::tasks_router};

pub fn root_router(db: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello" }))
        .nest("/auth", auth_router())
        .nest("/tasks", tasks_router())
        .route_layer(middleware::from_fn(auth))
        .layer(Extension(db))
}
