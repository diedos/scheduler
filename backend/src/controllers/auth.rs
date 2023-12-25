use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize)]
pub struct GoogleLoginResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct GoogleLoginParams {
    pub credential: String,
}

pub async fn google_login(
    State(db): State<Arc<PgPool>>,
    Json(payload): Json<GoogleLoginParams>,
) -> impl IntoResponse {
    let client = google_jwt_verify::Client::new(&CONFIG.google_oauth_client_id);
    let id_token = match client.verify_id_token(&payload.credential) {
        Ok(it) => it,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string())),
    };

    let user_id = id_token.get_claims().get_subject();

    let user = match auth::get_user_via_idp(State(db), IdentityProvider::Google, &user_id).await {
        Ok(user) => user,
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
    };

    // Example of extracting data from id_token, adjust as needed
    let email = id_token.get_payload().get_email();
    let name = id_token.get_payload().get_name();

    let result = GoogleLoginResponse {
        message: format!(
            "Hello, {}! Email {}, user_id {}",
            name,
            email,
            id_token.get_payload().get_picture_url()
        ),
    };

    return Ok((StatusCode::OK, Json(result)));
}
