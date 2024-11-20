use crate::queries::shared::Query as PgExtrasQuery;
use crate::web::routes::AppState;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::{
    all_locks, bloat, blocking, buffercache_stats, buffercache_usage, cache_hit, calls,
    connections, db_settings, duplicate_indexes, extensions, index_cache_hit, index_scans,
    index_size, index_usage, indexes, locks, long_running_queries, mandelbrot, null_indexes,
    outliers, records_rank, seq_scans, ssl_used, table_cache_hit, table_index_scans,
    table_indexes_size, table_size, tables, total_index_size, total_table_size, unused_indexes,
    vacuum_stats,
};
use askama_axum::Template;
use eyre::Result;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct QueryName {
    pub query_name: String,
}

#[derive(Template)]
#[template(path = "query.html")]
pub struct QueryTemplate {
    pub alert: Option<String>,
    pub query_name: String,
    pub query_data: Vec<Value>,
    pub version: String,
}

pub async fn show(State(state): State<AppState>, query: Query<QueryName>) -> impl IntoResponse {
    let query_name = query.0.query_name;

    let query_data = match get_data(&state.pool, &query_name).await {
        Ok(rows) => rows,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    QueryTemplate {
        version: env!("CARGO_PKG_VERSION").to_string(),
        alert: state.alert.lock().unwrap().clone(),
        query_name,
        query_data,
    }
    .into_response()
}

async fn get_data(pool: &Pool<Postgres>, query_name: &str) -> Result<Vec<Value>> {
    Ok(match query_name {
        "db_settings" => db_settings(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "bloat" => bloat(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "all_locks" => all_locks(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "blocking" => blocking(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "buffercache_stats" => buffercache_stats(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "buffercache_usage" => buffercache_usage(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "cache_hit" => cache_hit(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "calls" => calls(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "locks" => locks(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "connections" => connections(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "duplicate_indexes" => duplicate_indexes(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "extensions" => extensions(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "index_cache_hit" => index_cache_hit(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "index_scans" => index_scans(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "index_size" => index_size(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "index_usage" => index_usage(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "indexes" => indexes(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "long_running_queries" => long_running_queries(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "mandelbrot" => mandelbrot(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "null_indexes" => null_indexes(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "outliers" => outliers(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "records_rank" => records_rank(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "seq_scans" => seq_scans(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "ssl_used" => ssl_used(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "table_cache_hit" => table_cache_hit(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "table_index_scans" => table_index_scans(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "table_indexes_size" => table_indexes_size(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "table_size" => table_size(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "tables" => tables(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "total_index_size" => total_index_size(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "total_table_size" => total_table_size(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "unused_indexes" => unused_indexes(None, pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        "vacuum_stats" => vacuum_stats(pool)
            .await?
            .into_iter()
            .map(|r| r.to_json())
            .collect(),
        _ => eyre::bail!("Invalid query name: {}", query_name),
    })
}
