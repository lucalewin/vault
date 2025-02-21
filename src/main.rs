use std::sync::Arc;

use axum::Router;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod cipher;
mod error;
mod session;

#[derive(Debug)]
pub struct AppState {
    pub db: PgPool,
}

pub type App = Arc<AppState>;

impl AppState {
    pub async fn new() -> Self {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .expect("Failed to connect to database");
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        Self { db: pool }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // set up tracing debug level
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_http=debug,axum=trace,vault=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let state = AppState::new().await;

    let app = Router::new()
        .nest_service("/", ServeDir::new("static").append_index_html_on_directories(true))
        .nest("/api/v1", api::router())
        .with_state(Arc::new(state))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
