use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::State, Json};
use chacha20poly1305::aead::OsRng;
use rand::RngCore;
use serde::Deserialize;

use crate::{
    cipher::{derive_key, encrypt_data},
    App,
};

#[derive(Deserialize)]
pub struct RegisterPayload {
    username: String,
    email: String,
    password: String,
}

pub async fn register(State(app): State<App>, Json(payload): Json<RegisterPayload>) -> String {
    tracing::trace!("register new user");

    // create the hash for the master and recovery keys
    let mut master_salt = [0u8; 16];
    rand::rng().fill_bytes(&mut master_salt);
    let mut recovery_salt = [0u8; 16];
    rand::rng().fill_bytes(&mut recovery_salt);

    // create a random recovery phrase
    let mut recovery_phrase = [0u8; 48];
    rand::rng().fill_bytes(&mut recovery_phrase);

    // derive the master and recovery keys
    let master_key = derive_key(payload.password.as_bytes(), &master_salt, 32);
    let recovery_key = derive_key(&recovery_phrase, &recovery_salt, 32);

    // encrypt the master key with the recovery key
    let (encrypted_key, nonce) = encrypt_data(&master_key, &recovery_key);
    let recovery_code = [nonce, encrypted_key].concat();

    // hash the password (for login verification)
    let password_salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &password_salt)
        .unwrap();

    // hash the recovery phrase (for recovery verification)
    let recovery_phrase_salt = SaltString::generate(&mut OsRng);
    let recovery_phrase_hash = Argon2::default()
        .hash_password(&recovery_phrase, &recovery_phrase_salt)
        .unwrap();

    // insert into database
    sqlx::query!(
        r#"
        INSERT INTO users (username, email, password_hash, master_salt, recovery_salt, recovery_phrase_hash, recovery_code)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        payload.username,
        payload.email,
        password_hash.to_string(),
        &master_salt,
        &recovery_salt,
        recovery_phrase_hash.to_string(),
        recovery_code
    )
    .execute(&app.db)
    .await
    .expect("Failed to insert user into database");

    // send to user: hex-encoded recovery phrase
    hex::encode(recovery_phrase)
}
