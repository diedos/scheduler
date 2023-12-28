use axum::{
    routing::{get, post},
    Router,
};

use crate::controllers::auth::*;

pub fn auth_router() -> Router {
    Router::new()
        .route("/google/api", get(google_api_authorization))
        .route("/google/login", post(google_login))
}
