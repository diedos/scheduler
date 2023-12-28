use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use google_jwt_verify::{IdPayload, Token};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    models::auth::{get_user_via_idp, IdentityProvider},
    CONFIG,
};

struct UserIdP {
    pub user_provider_id: String,
    pub provider: IdentityProvider,
    pub user_given_name: String,
    pub user_email: String,
}

// #[async_trait]
// impl<S, B> FromRequest<S, B> for UserIdP
// where
//     S: Send + Sync,
//     B: Send + 'static,
// {
//     type Rejection = (StatusCode, String);

//     async fn from_request(request: Request<B>, ) -> Result<Self, Self::Rejection> {
//         let token = extract_token(&request).map_err(|err| (StatusCode::UNAUTHORIZED, err))?;
//         let jwt = verify_jwt(&token).map_err(|err| (StatusCode::UNAUTHORIZED, err.to_string()))?;

//         let provider = match jwt.get_claims().get_issuer().as_str() {
//             "https://accounts.google.com" => IdentityProvider::Google,
//             _ => return Err((StatusCode::UNAUTHORIZED, "Invalid JWT issuer".to_string())),
//         };

//         Ok(UserIdP {
//             user_provider_id: jwt.get_claims().get_subject().to_string(),
//             provider,
//         })
//     }
// }

pub async fn auth(mut request: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    let token = match extract_token(&request) {
        Ok(it) => it,
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.1)),
    };

    let jwt = match verify_jwt(&token) {
        Ok(it) => it,
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
    };

    let provider = match jwt.get_claims().get_issuer().as_str() {
        "https://accounts.google.com" => IdentityProvider::Google,
        _ => return Err((StatusCode::UNAUTHORIZED, "Invalid JWT issuer".to_string())),
    };

    let user_idp = UserIdP {
        user_provider_id: jwt.get_claims().get_subject().to_string(),
        provider,
        user_given_name: jwt.get_payload().get_given_name().to_string(),
        user_email: jwt.get_payload().get_email().to_string(),
    };

    let db = match request.extensions().get::<Arc<PgPool>>().cloned() {
        Some(db) => db,
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error".to_string(),
            ))
        }
    };

    // TODO: Cache user data. Now we are fetching it from the database on every request.
    let get_user = match get_user_via_idp(
        State(db),
        user_idp.provider,
        &user_idp.user_provider_id,
        &user_idp.user_email,
        &user_idp.user_given_name,
    )
    .await
    {
        Ok(user) => user,
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
    };

    request.extensions_mut().insert(get_user);

    let response = next.run(request).await;
    Ok(response)
}

// pub async fn auth<B>(
//     mut request: Request<B>,
//     next: Next<B>,
// ) -> Result<Response, (StatusCode, String)> {
//     let user_idp: UserIdP = UserIdP::from_request(&mut RequestParts::new(request)).await?;

//     let db = request.extensions().get::<Arc<PgPool>>().cloned().unwrap();

//     // TODO: Cache user data. Now we are fetching it from the database on every request.
//     let get_user =
//         match get_user_via_idp(State(db), user_idp.provider, &user_idp.user_provider_id).await {
//             Ok(user) => user,
//             Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
//         };

//     request.extensions().insert(get_user);

//     let response = next.run(request).await;
//     Ok(response)
// }

fn extract_token<B>(request: &Request<B>) -> Result<String, (StatusCode, String)> {
    let auth_header = request.headers().get("Authorization");

    if auth_header.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Missing Authorization header".to_string(),
        ));
    }

    let auth_header_str = match auth_header.unwrap().to_str() {
        Ok(it) => it,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header".to_string(),
            ))
        }
    };

    if !auth_header_str.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid Authorization header".to_string(),
        ));
    }

    Ok(auth_header_str.split(" ").collect::<Vec<&str>>()[1].to_string())
}

fn verify_jwt(token: &str) -> Result<Token<IdPayload>, String> {
    let client = google_jwt_verify::Client::new(&CONFIG.google_oauth_client_id);
    match client.verify_id_token(&token) {
        Ok(it) => return Ok(it),
        Err(_) => return Err("Invalid JWT".to_string()),
    };
}
