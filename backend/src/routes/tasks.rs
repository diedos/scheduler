use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{controllers::tasks::*, middlewares::auth::auth};

pub fn tasks_router() -> Router {
    Router::new()
        .route("/", get(get_tasks).post(create_task))
        .route("/:id", get(get_task).delete(delete_task))
        .route("/:id/complete", post(complete_task))
        .route_layer(middleware::from_fn(auth))
}
