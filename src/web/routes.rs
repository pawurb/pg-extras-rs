use axum::{routing::get, Router};
use std::sync::{Arc, Mutex};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use super::controllers;
use sqlx::PgPool;
use tower_http::services::ServeDir;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub alert: Arc<Mutex<Option<String>>>,
}

pub async fn app(pool: PgPool) -> Router {
    let state = AppState {
        pool,
        alert: Arc::new(Mutex::new(None)),
    };

    tracing_subscriber::fmt::init();

    Router::new()
        .route("/pg_extras", get(controllers::home::home))
        .route("/pg_extras/queries", get(controllers::queries::show))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state)
}
