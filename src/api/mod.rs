use axum::{
    routing::{get, post},
    Router,
};

use crate::App;

pub mod auth;
pub mod credentials;

pub fn router() -> Router<App> {
    Router::new()
        // auth sub-routes
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        // credentials sub-routes
        .route(
            "/credentials",
            get(credentials::get_all_credentials).post(credentials::add_credential),
        )
        .route(
            "/credentials/{id}",
            post(credentials::get_credential)
                .put(credentials::update_credential)
                .delete(credentials::delete_credential),
        )
}
