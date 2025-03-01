use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    cipher::{decrypt_data, derive_key, encrypt_data},
    error::Error,
    session::SessionUser,
    App,
};

#[derive(Deserialize)]
pub struct AddRequest {
    // not related to new entry
    // pub email: String,
    pub master_password: String,

    // data for new entry
    pub service: String,
    pub username: String,
    pub password: String,
}

pub async fn add_credential(
    State(app): State<App>,
    SessionUser(id): SessionUser,
    Json(payload): Json<AddRequest>,
) -> Result<String, Error> {
    tracing::info!("Adding credential for user {}", id);
    // verify user by email and hash of master password
    let user = sqlx::query!(
        r#"
        SELECT password_hash, master_salt FROM users WHERE id = $1
        "#,
        id
    )
    .fetch_one(&app.db)
    .await
    .expect("Failed to fetch user from database");

    // // verify master password hash
    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    Argon2::default()
        .verify_password(payload.master_password.as_bytes(), &parsed_hash)
        // if the password is incorrect, return an error
        .map_err(|_| Error::Generic("Invalid master password".to_string()))?;

    tracing::info!("Master password verified");

    // encrypt the password with the master key
    let encrypted_password = {
        let key = derive_key(payload.master_password.as_bytes(), &user.master_salt, 32);
        let (ciphertext, nonce) = encrypt_data(payload.password.as_bytes(), &key);
        [nonce, ciphertext].concat()
    };

    // insert into database
    sqlx::query!(
        r#"
        INSERT INTO credentials (user_id, service, username, password)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        payload.service,
        payload.username,
        encrypted_password
    )
    .execute(&app.db)
    .await
    .expect("Failed to insert password into database");

    Ok(serde_json::json!({
        "status": "success",
        "message": "Password added successfully",
    })
    .to_string())
}

// #[derive(Deserialize)]
// pub struct GetAllRequest {
//     pub email: String,
//     pub master_password: String,
// }

#[derive(Serialize, Deserialize)]
struct SmallEntry {
    id: i32,
    service: String,
    username: String,
}

pub async fn get_all_credentials(
    State(app): State<App>,
    SessionUser(id): SessionUser,
) -> Result<String, Error> {
    // get all credentials for the user
    let credentials = sqlx::query_as!(
        SmallEntry,
        "SELECT id, service, username FROM credentials WHERE user_id = $1",
        id
    )
    .fetch_all(&app.db)
    .await
    .expect("Failed to fetch credentials from database");

    Ok(serde_json::json!({
        "status": "success",
        "credentials": credentials,
    })
    .to_string())
}

#[derive(Deserialize)]
pub struct GetRequest {
    // pub email: String,
    pub master_password: String,
}

pub async fn get_credential(
    Path(cred_id): Path<i32>,
    State(app): State<App>,
    SessionUser(user_id): SessionUser,
    Json(payload): Json<GetRequest>,
) -> Result<String, Error> {
    // verify user by email and hash of master password
    let user = sqlx::query!(
        "SELECT password_hash, master_salt FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&app.db)
    .await
    .expect("Failed to fetch user from database");

    // verify master password hash
    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    Argon2::default()
        .verify_password(payload.master_password.as_bytes(), &parsed_hash)
        .map_err(|_| Error::Generic("Invalid master password".to_string()))?;

    // get the credential for the user
    let credential = sqlx::query!(
        "SELECT service, username, password FROM credentials WHERE user_id = $1 AND id = $2",
        user_id,
        cred_id
    )
    .fetch_one(&app.db)
    .await
    .expect("Failed to fetch credential from database");

    // decrypt password
    let decrypted_password = {
        let key = derive_key(payload.master_password.as_bytes(), &user.master_salt, 32);
        let nonce = &credential.password[..12];
        let ciphertext = &credential.password[12..];
        let plaintext = decrypt_data(ciphertext, nonce, &key);
        String::from_utf8(plaintext).unwrap()
    };

    Ok(serde_json::json!({
        "status": "success",
        "credential": {
            "service": credential.service,
            "username": credential.username,
            "password": decrypted_password,
        }
    })
    .to_string())
}

pub async fn update_credential() {}

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub email: String,
    pub master_password: String,
}

pub async fn delete_credential(
    Path(id): Path<i32>,
    State(app): State<App>,
    Json(payload): Json<DeleteRequest>,
) -> Result<String, Error> {
    // verify user by email and hash of master password
    let user = sqlx::query!(
        "SELECT id, password_hash, master_salt FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&app.db)
    .await
    .expect("Failed to fetch user from database");

    // verify master password hash
    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    Argon2::default()
        .verify_password(payload.master_password.as_bytes(), &parsed_hash)
        .map_err(|_| Error::Generic("Invalid master password".to_string()))?;

    // delete the credential
    sqlx::query!(
        "DELETE FROM credentials WHERE user_id = $1 AND id = $2",
        user.id,
        id
    )
    .execute(&app.db)
    .await
    .expect("Failed to delete credential from database");

    Ok(serde_json::json!({
        "status": "success",
        "message": "Credential deleted successfully",
    })
    .to_string())
}
