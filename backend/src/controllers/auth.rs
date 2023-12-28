use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    models::auth::{self, IdentityProvider},
    CONFIG,
};

#[derive(Deserialize)]
pub struct GoogleAuthParams {
    #[serde(default)]
    code: String,
    #[serde(default)]
    scope: String,
}

pub async fn google_api_authorization(Query(query): Query<GoogleAuthParams>) -> Redirect {
    let google_oauth_uri: String = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}/auth/google&response_type=code&scope=openid",
        &CONFIG.google_oauth_client_id, &CONFIG.api_root_url
    );

    if query.code.is_empty() || query.scope.is_empty() {
        return Redirect::temporary(google_oauth_uri.as_str());
    }

    if !query.scope.contains("openid") {
        return Redirect::temporary(&CONFIG.api_root_url);
    }

    let token_request_body = serde_json::json!({
        "code": query.code,
        "client_id": &CONFIG.google_oauth_client_id,
        "client_secret": &CONFIG.google_oauth_client_secret,
        "redirect_uri": format!("{}/auth/google", &CONFIG.api_root_url),
        "grant_type": "authorization_code",
    });

    let _token_response = reqwest::Client::new()
        .post("https://oauth2.googleapis.com/token")
        .body(token_request_body.to_string())
        .send()
        .await;

    // TODO: use the token

    return Redirect::temporary(&CONFIG.api_root_url);
}

#[derive(Deserialize)]
pub struct GoogleLoginParams {
    pub credential: String,
}

pub async fn google_login(
    Extension(db): Extension<Arc<PgPool>>,
    Json(payload): Json<GoogleLoginParams>,
) -> impl IntoResponse {
    let client = google_jwt_verify::Client::new(&CONFIG.google_oauth_client_id);
    let id_token = match client.verify_id_token(&payload.credential) {
        Ok(it) => it,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string())),
    };

    let user_id = id_token.get_claims().get_subject();

    let user_email = id_token.get_payload().get_email(); // TODO: create a registration flow and move this elsewhere
    let user_name = id_token.get_payload().get_given_name(); // TODO: create a registration flow and move this elsewhere

    let _user = match auth::get_user_via_idp(
        State(db),
        IdentityProvider::Google,
        &user_id,
        &user_email,
        &user_name,
    )
    .await
    {
        Ok(user) => user,
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
    };

    return Ok(StatusCode::OK);
}
