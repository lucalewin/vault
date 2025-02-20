use std::sync::Arc;

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::{extract::State, Json};
use chacha20poly1305::aead::OsRng;
use rand::RngCore;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    cipher::{decrypt_data, derive_key, encrypt_data},
    error::{Error, Result},
    App,
};

// #[derive(Debug)]
// pub struct AppState {
//     pub db: PgPool,
// }

// impl AppState {
//     pub async fn new() -> Self {
//         let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

//         let pool = PgPoolOptions::new()
//             .max_connections(5)
//             .connect(&url)
//             .await
//             .expect("Failed to connect to database");
//         sqlx::migrate!()
//             .run(&pool)
//             .await
//             .expect("Failed to run migrations");

//         Self { db: pool }
//     }
// }

// #[derive(Deserialize)]
// pub struct RegisterPayload {
//     username: String,
//     email: String,
//     password: String,
// }

// #[derive(Deserialize)]
// pub struct LoginPayload {}

// #[derive(Deserialize)]
// pub struct RecoveryPayload {}

// pub async fn register(
//     State(app): State<Arc<AppState>>,
//     Json(payload): Json<RegisterPayload>,
// ) -> String {
//     tracing::trace!("register new user");

//     // create the hash for the master and recovery keys
//     let mut master_salt = [0u8; 16];
//     rand::rng().fill_bytes(&mut master_salt);
//     let mut recovery_salt = [0u8; 16];
//     rand::rng().fill_bytes(&mut recovery_salt);

//     // create a random recovery phrase
//     let mut recovery_phrase = [0u8; 48];
//     rand::rng().fill_bytes(&mut recovery_phrase);

//     // derive the master and recovery keys
//     let master_key = derive_key(payload.password.as_bytes(), &master_salt, 32);
//     let recovery_key = derive_key(&recovery_phrase, &recovery_salt, 32);

//     // encrypt the master key with the recovery key
//     let (encrypted_key, nonce) = encrypt_data(&master_key, &recovery_key);
//     let recovery_code = [nonce, encrypted_key].concat();

//     // hash the password (for login verification)
//     let password_salt = SaltString::generate(&mut OsRng);
//     let password_hash = Argon2::default()
//         .hash_password(payload.password.as_bytes(), &password_salt)
//         .unwrap();

//     // hash the recovery phrase (for recovery verification)
//     let recovery_phrase_salt = SaltString::generate(&mut OsRng);
//     let recovery_phrase_hash = Argon2::default()
//         .hash_password(&recovery_phrase, &recovery_phrase_salt)
//         .unwrap();

//     // insert into database
//     sqlx::query!(
//         r#"
//         INSERT INTO users (username, email, password_hash, master_salt, recovery_salt, recovery_phrase_hash, recovery_code)
//         VALUES ($1, $2, $3, $4, $5, $6, $7)
//         "#,
//         payload.username,
//         payload.email,
//         password_hash.to_string(),
//         &master_salt,
//         &recovery_salt,
//         recovery_phrase_hash.to_string(),
//         recovery_code
//     )
//     .execute(&app.db)
//     .await
//     .expect("Failed to insert user into database");

//     // send to user: hex-encoded recovery phrase
//     hex::encode(recovery_phrase)
// }

// HERE

// #[derive(Deserialize)]
// pub struct AddPasswordPayload {
//     email: String,
//     master_password: String,
//     url: String,
//     username: String,
//     password: String,
// }

// pub async fn add_password(
//     State(app): State<App>,
//     Json(payload): Json<AddPasswordPayload>,
// ) -> Result<()> {
//     tracing::trace!("add new password");
//     // get current user from database and verify master_password
//     let user = sqlx::query!(
//         r#"
//         SELECT id, password_hash, master_salt FROM users WHERE email = $1
//         "#,
//         payload.email
//     )
//     .fetch_one(&app.db)
//     .await
//     .expect("Failed to fetch user from database");

//     let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
//     Argon2::default()
//         .verify_password(payload.master_password.as_bytes(), &parsed_hash)
//         // if the password is incorrect, return an error
//         .map_err(|_| Error::Generic("Invalid master password".to_string()))?;

//     let key = derive_key(payload.master_password.as_bytes(), &user.master_salt, 32);
//     let (ciphertext, nonce) = encrypt_data(payload.password.as_bytes(), &key);
//     let encrypted_password = [nonce, ciphertext].concat();

//     // insert into database
//     sqlx::query!(
//         r#"
//         INSERT INTO passwords (user_id, url, username, password)
//         VALUES ($1, $2, $3, $4)
//         "#,
//         user.id,
//         payload.url,
//         payload.username,
//         hex::encode(encrypted_password)
//     )
//     .execute(&app.db)
//     .await
//     .expect("Failed to insert password into database");

//     Ok(())
// }

// #[derive(Deserialize)]
// pub struct GetPasswordPayload {
//     email: String,
//     master_password: String,
//     url: String,
// }

// pub async fn get_password(
//     State(app): State<App>,
//     Json(payload): Json<GetPasswordPayload>,
// ) -> Result<String> {
//     tracing::trace!("get password");
//     // get current user from database and verify master_password
//     let user = sqlx::query!(
//         r#"
//         SELECT id, password_hash, master_salt FROM users WHERE email = $1
//         "#,
//         payload.email
//     )
//     .fetch_one(&app.db)
//     .await
//     .expect("Failed to fetch user from database");

//     let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
//     Argon2::default()
//         .verify_password(payload.master_password.as_bytes(), &parsed_hash)
//         // if the password is incorrect, return an error
//         .map_err(|_| Error::Generic("Invalid master password".to_string()))?;

//     // get the encrypted password from the database
//     let encrypted_password = sqlx::query!(
//         r#"
//         SELECT password FROM passwords WHERE user_id = $1 AND url = $2
//         "#,
//         user.id,
//         payload.url
//     )
//     .fetch_one(&app.db)
//     .await
//     .map_err(|_| Error::Generic("Password not found".to_string()))?
//     .password;

//     let encoded = hex::decode(encrypted_password).unwrap();

//     let nonce = &encoded[..12];
//     let ciphertext = &encoded[12..];
//     let key = derive_key(payload.master_password.as_bytes(), &user.master_salt, 32);
//     let plaintext = decrypt_data(ciphertext, nonce, &key);

//     Ok(String::from_utf8(plaintext).unwrap())
// }

// pub async fn login() {
//     todo!()
// }

// pub async fn recovery() {
//     todo!()
// }
