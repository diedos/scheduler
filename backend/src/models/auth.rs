use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::NaiveDateTime, FromRow, PgPool};

use crate::utils::serializers::serialize_dt;

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    #[serde(serialize_with = "serialize_dt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(serialize_with = "serialize_dt")]
    pub updated_at: Option<NaiveDateTime>,
    pub display_name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "identity_provider", rename_all = "lowercase")]
pub enum IdentityProvider {
    Google,
}

pub async fn get_user_via_idp(
    State(db): State<Arc<PgPool>>,
    provider: IdentityProvider,
    provider_user_id: &str,
) -> Result<User, String> {
    let query = sqlx::query_as::<_, User>(
        "SELECT * FROM users
        INNER JOIN users_idp ON users.id = users_idp.user_id
        WHERE users_idp.provider = $1 AND users_idp.provider_user_id = $2",
    )
    .bind(&provider)
    .bind(&provider_user_id)
    .fetch_optional(&*db)
    .await;

    let user = match query {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                let new_user = register_user(State(db.clone()), "New user", "placeholder").await?;
                create_user_idp(State(db), new_user.id, provider, &provider_user_id).await?;
                new_user
            }
        },
        Err(err) => return Err(err.to_string()),
    };

    Ok(user)
}

async fn create_user_idp(
    State(db): State<Arc<PgPool>>,
    user_id: i32,
    provider: IdentityProvider,
    provider_user_id: &str,
) -> Result<bool, String> {
    let user_idp = sqlx::query(
        "INSERT INTO users_idp (user_id, provider, provider_user_id) VALUES ($1, $2::identity_provider, $3)",
    )
    .bind(&user_id)
    .bind(&provider)
    .bind(&provider_user_id)
    .execute(&*db)
    .await;

    match user_idp {
        Ok(_) => {
            println!(
                "Created user_idp: {:?}, {:?}, {:?}",
                &user_id, &provider, &provider_user_id
            );
            Ok(true)
        }
        Err(err) => {
            println!(
                "Failed to create user_idp: {:?}, {:?}, {:?}. Error: {:?}",
                &user_id, &provider, &provider_user_id, err
            );
            Err(err.to_string())
        }
    }
}

async fn register_user(
    State(db): State<Arc<PgPool>>,
    display_name: &str,
    email: &str,
) -> Result<User, String> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (display_name, email) VALUES ($1, $2) RETURNING *",
    )
    .bind(display_name)
    .bind(email)
    .fetch_one(&*db)
    .await;

    match user {
        Ok(user) => {
            println!(
                "Created user: {:?}, {:?}, {:?}",
                user.id, user.display_name, user.email
            );
            Ok(user)
        }
        Err(err) => {
            println!("Failed to create user: {:?}, {:?}", &display_name, &email);
            Err(err.to_string())
        }
    }
}
