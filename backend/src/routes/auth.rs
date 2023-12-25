use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::controllers::auth::*;

pub fn auth_router() -> Router<Arc<PgPool>> {
    Router::new()
        .route("/google/api", get(google_api_authorization))
        .route("/google/login", post(google_login))
}
