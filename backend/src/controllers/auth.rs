use axum::response::Redirect;

use crate::CONFIG;

pub async fn login_google_oauth() -> Redirect {
    let uri = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}/login&response_type=code&scope=openid%20email",
        &CONFIG.google_oauth_client_id, &CONFIG.root_url
    );
    Redirect::temporary(uri.as_str())
}
