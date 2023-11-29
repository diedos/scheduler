use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::controllers::auth::*;

pub fn auth_router() -> Router<Arc<PgPool>> {
    Router::new().route("/google", get(google_api_authorization))
}
