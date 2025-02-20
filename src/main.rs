use std::sync::Arc;

use axum::{routing::post, Router};
use routes::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cipher;
mod error;
mod routes;
mod util;

#[tokio::main]
async fn main() {
    // set up tracing debug level
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_http=debug,axum::rejection=trace,vault=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let state = AppState::new().await;

    let app = Router::new()
        .route("/register", post(routes::register))
        .route("/login", post(routes::login))
        .route("/recovery", post(routes::recovery))
        .route(
            "/password",
            post(routes::get_password).put(routes::add_password),
        )
        .with_state(Arc::new(state));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
