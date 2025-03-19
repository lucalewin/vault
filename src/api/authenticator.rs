use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use totp_rs::{Algorithm, Secret, TOTP};

use crate::{error::Error, session::SessionUser, App};

pub async fn get_codes(
    State(app): State<App>,
    SessionUser(user_id): SessionUser,
) -> Result<String, Error> {
    let codes = sqlx::query!(
        "SELECT id, username, service_name, secret_key FROM authenticator_codes WHERE user_id = $1",
        user_id
    )
    .fetch_all(&app.db)
    .await
    .unwrap();

    #[derive(Serialize)]
    struct AuthenticationEntry {
        id: i32,
        username: String,
        service: String,
        code: String,
    }

    let entries = codes
        .into_iter()
        .map(|c| {
            let totp = TOTP::new_unchecked(
                Algorithm::SHA1,
                6,
                1,
                30,
                Secret::Encoded(c.secret_key).to_bytes().unwrap(),
            );
            let token = totp.generate_current().unwrap();

            AuthenticationEntry {
                id: c.id,
                username: c.username,
                service: c.service_name,
                code: token,
            }
        })
        .collect::<Vec<_>>();

    Ok(Json(serde_json::json!(entries)).to_string())
}

#[derive(Deserialize)]
pub struct InsertCodePayload {
    username: String,
    service: String,
    secret: String,
}

pub async fn insert_code(
    State(app): State<App>,
    SessionUser(user_id): SessionUser,
    Json(payload): Json<InsertCodePayload>,
) -> Result<String, Error> {
    sqlx::query!(
        "INSERT INTO authenticator_codes (user_id, username, service_name, secret_key) VALUES ($1, $2, $3, $4)",
        user_id,
        payload.username,
        payload.service,
        payload.secret
    )
    .execute(&app.db)
    .await
    .unwrap();

    Ok(Json(serde_json::json!({"status": "success"})).to_string())
}
