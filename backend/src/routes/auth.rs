use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::controllers::auth::login_google_oauth;

pub fn auth_router() -> Router<Arc<PgPool>> {
    Router::new().route("/google", get(login_google_oauth))
}
