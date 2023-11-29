use axum::{extract::Query, response::Redirect};
use reqwest;
use serde::Deserialize;

use crate::CONFIG;

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
