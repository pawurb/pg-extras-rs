use crate::diagnose;
use crate::diagnose::run::CheckResult;
use crate::web::routes::AppState;
use askama_axum::Template;
use axum::{extract::State, response::IntoResponse};
use eyre::Result;
use reqwest::StatusCode;
use sqlx::{Pool, Postgres};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub alert: Option<String>,
    pub query_name: String,
    pub checks_result: Vec<CheckResult>,
    pub version: String,
}

pub async fn home(State(state): State<AppState>) -> impl IntoResponse {
    let checks_result = match get_data(&state.pool).await {
        Ok(data) => data,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    HomeTemplate {
        version: env!("CARGO_PKG_VERSION").to_string(),
        alert: state.alert.lock().unwrap().clone(),
        query_name: "diagnose".to_string(),
        checks_result,
    }
    .into_response()
}

async fn get_data(pool: &Pool<Postgres>) -> Result<Vec<CheckResult>> {
    Ok(diagnose(pool).await?)
}
