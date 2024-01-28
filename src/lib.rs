use std::collections::HashMap;
use std::env;
pub mod queries;
pub use queries::all_locks::AllLocks;
pub use queries::bloat::Bloat;
pub use queries::blocking::Blocking;
pub use queries::buffercache_stats::BuffercacheStats;
pub use queries::buffercache_usage::BuffercacheUsage;
pub use queries::cache_hit::CacheHit;
pub use queries::calls::Calls;
pub use queries::connections::Connections;
pub use queries::db_settings::DbSettings;
pub use queries::duplicate_indexes::DuplicateIndexes;
pub use queries::extensions::Extensions;
pub use queries::index_cache_hit::IndexCacheHit;
pub use queries::index_scans::IndexScans;
pub use queries::index_size::IndexSize;
pub use queries::index_usage::IndexUsage;
pub use queries::indexes::Indexes;
pub use queries::locks::Locks;
pub use queries::long_running_queries::LongRunningQueries;
pub use queries::mandelbrot::Mandelbrot;
pub use queries::null_indexes::NullIndexes;
pub use queries::outliers::Outliers;
pub use queries::records_rank::RecordsRank;
pub use queries::seq_scans::SeqScans;
pub use queries::shared::{get_default_schema, Query};
pub use queries::ssl_used::SslUsed;
pub use queries::table_cache_hit::TableCacheHit;
pub use queries::table_index_scans::TableIndexScans;
pub use queries::table_indexes_size::TableIndexesSize;
pub use queries::table_size::TableSize;
pub use queries::tables::Tables;
pub use queries::total_index_size::TotalIndexSize;
pub use queries::total_table_size::TotalTableSize;
pub use queries::unused_indexes::UnusedIndexes;
pub use queries::vacuum_stats::VacuumStats;
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;

#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

pub fn render_table<T: Query>(items: Vec<T>) {
    let mut table = Table::new();
    table.add_row(T::headers());

    let columns_count = T::headers().len();

    for item in items {
        table.add_row(item.to_row());
    }
    table.set_titles(Row::new(vec![
        Cell::new(T::description().as_str()).style_spec(format!("H{}", columns_count).as_str())
    ]));
    table.printstd();
}

pub async fn bloat() -> Result<Vec<Bloat>, PgExtrasError> {
    get_rows(None).await
}

pub async fn blocking(limit: Option<String>) -> Result<Vec<Blocking>, PgExtrasError> {
    get_rows(Some(limit_params(limit))).await
}

pub async fn calls(limit: Option<String>) -> Result<Vec<Calls>, PgExtrasError> {
    get_rows(Some(limit_params(limit))).await
}

pub async fn extensions() -> Result<Vec<Extensions>, PgExtrasError> {
    get_rows(None).await
}

pub async fn table_cache_hit() -> Result<Vec<TableCacheHit>, PgExtrasError> {
    get_rows(None).await
}

pub async fn tables(schema: Option<String>) -> Result<Vec<Tables>, PgExtrasError> {
    get_rows(Some(schema_params(schema))).await
}

pub async fn index_cache_hit(schema: Option<String>) -> Result<Vec<IndexCacheHit>, PgExtrasError> {
    get_rows(Some(schema_params(schema))).await
}

pub async fn indexes() -> Result<Vec<Indexes>, PgExtrasError> {
    get_rows(None).await
}

pub async fn index_size() -> Result<Vec<IndexSize>, PgExtrasError> {
    get_rows(None).await
}

pub async fn index_usage(schema: Option<String>) -> Result<Vec<IndexUsage>, PgExtrasError> {
    get_rows(Some(schema_params(schema))).await
}

pub async fn index_scans(schema: Option<String>) -> Result<Vec<IndexScans>, PgExtrasError> {
    get_rows(Some(schema_params(schema))).await
}

pub async fn null_indexes(
    min_relation_size_mb: Option<String>,
) -> Result<Vec<NullIndexes>, PgExtrasError> {
    let min_relation_size_mb = min_relation_size_mb.unwrap_or("10".to_string());

    let params: HashMap<String, String> = [(
        "min_relation_size_mb".to_string(),
        min_relation_size_mb.to_string(),
    )]
    .iter()
    .cloned()
    .collect();
    get_rows(Some(params)).await
}

pub async fn locks() -> Result<Vec<Locks>, PgExtrasError> {
    get_rows(None).await
}

pub async fn all_locks() -> Result<Vec<AllLocks>, PgExtrasError> {
    get_rows(None).await
}

pub async fn long_running_queries() -> Result<Vec<LongRunningQueries>, PgExtrasError> {
    get_rows(None).await
}

pub async fn mandelbrot() -> Result<Vec<Mandelbrot>, PgExtrasError> {
    get_rows(None).await
}

pub async fn outliers() -> Result<Vec<Outliers>, PgExtrasError> {
    get_rows(None).await
}

pub async fn records_rank(schema: Option<String>) -> Result<Vec<RecordsRank>, PgExtrasError> {
    get_rows(Some(schema_params(schema))).await
}

pub async fn seq_scans(schema: Option<String>) -> Result<Vec<SeqScans>, PgExtrasError> {
    get_rows(Some(schema_params(schema))).await
}

pub async fn table_index_scans(
    schema: Option<String>,
) -> Result<Vec<TableIndexScans>, PgExtrasError> {
    get_rows(Some(schema_params(schema))).await
}

pub async fn table_indexes_size(
    schema: Option<String>,
) -> Result<Vec<TableIndexesSize>, PgExtrasError> {
    get_rows(Some(schema_params(schema))).await
}

pub async fn table_size() -> Result<Vec<TableSize>, PgExtrasError> {
    get_rows(None).await
}

pub async fn total_index_size() -> Result<Vec<TotalIndexSize>, PgExtrasError> {
    get_rows(None).await
}

pub async fn total_table_size() -> Result<Vec<TotalTableSize>, PgExtrasError> {
    get_rows(None).await
}

pub async fn unused_indexes(schema: Option<String>) -> Result<Vec<UnusedIndexes>, PgExtrasError> {
    get_rows(Some(schema_params(schema))).await
}

pub async fn duplicate_indexes() -> Result<Vec<DuplicateIndexes>, PgExtrasError> {
    get_rows(None).await
}

pub async fn vacuum_stats() -> Result<Vec<VacuumStats>, PgExtrasError> {
    get_rows(None).await
}

pub async fn buffercache_stats() -> Result<Vec<BuffercacheStats>, PgExtrasError> {
    get_rows(None).await
}

pub async fn buffercache_usage() -> Result<Vec<BuffercacheUsage>, PgExtrasError> {
    get_rows(None).await
}

pub async fn ssl_used() -> Result<Vec<SslUsed>, PgExtrasError> {
    get_rows(None).await
}

pub async fn connections() -> Result<Vec<Connections>, PgExtrasError> {
    get_rows(None).await
}

pub async fn cache_hit(schema: Option<String>) -> Result<Vec<CacheHit>, PgExtrasError> {
    get_rows(Some(schema_params(schema))).await
}

pub async fn db_settings() -> Result<Vec<DbSettings>, PgExtrasError> {
    get_rows(None).await
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PgExtrasError {
    #[error("Both $DATABASE_URL and $PG_EXTRAS_DATABASE_URL are not set.")]
    MissingConfigVars(),
    #[error("Cannot connect to database: '{0}'")]
    DbConnectionError(String),
    #[error("Unknown pg-extras error: '{0}'")]
    Unknown(String),
}

async fn get_rows<T: Query>(
    params: Option<HashMap<String, String>>,
) -> Result<Vec<T>, PgExtrasError> {
    let mut query = T::read_file();

    if let Some(params) = params {
        for (key, value) in &params {
            query = query.replace(&format!("%{{{}}}", key), value.as_str());
        }
    }

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url()?.as_str())
        .await
    {
        Ok(pool) => pool,
        Err(e) => return Err(PgExtrasError::DbConnectionError(format!("{}", e))),
    };

    Ok(match sqlx::query(&query).fetch_all(&pool).await {
        Ok(rows) => rows.iter().map(T::new).collect(),
        Err(e) => return Err(PgExtrasError::Unknown(format!("{}", e))),
    })
}

fn db_url() -> Result<String, PgExtrasError> {
    env::var("PG_EXTRAS_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .map_err(|_| PgExtrasError::MissingConfigVars())
}

fn schema_params(schema_name: Option<String>) -> HashMap<String, String> {
    let schema_name = schema_name.unwrap_or(get_default_schema());
    [("schema".to_string(), schema_name.to_string())]
        .iter()
        .cloned()
        .collect()
}

fn limit_params(limit: Option<String>) -> HashMap<String, String> {
    let limit = limit.unwrap_or("10".to_string());
    [("limit".to_string(), limit.to_string())]
        .iter()
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup() -> Result<(), Box<dyn std::error::Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url()?.as_str())
            .await?;

        for extension in ["sslinfo", "pg_stat_statements", "pg_buffercache"] {
            let query = format!("CREATE EXTENSION IF NOT EXISTS {};", extension);
            sqlx::query(&query).execute(&pool).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        setup().await?;

        render_table(cache_hit(None).await?);
        render_table(bloat().await?);
        render_table(blocking(None).await?);
        render_table(calls(None).await?);
        render_table(extensions().await?);
        render_table(table_cache_hit().await?);
        render_table(seq_scans(None).await?);
        render_table(table_index_scans(None).await?);
        render_table(table_indexes_size(None).await?);
        render_table(tables(None).await?);
        render_table(index_cache_hit(None).await?);
        render_table(indexes().await?);
        render_table(index_size().await?);
        render_table(index_usage(None).await?);
        render_table(index_scans(None).await?);
        render_table(null_indexes(None).await?);
        render_table(locks().await?);
        render_table(all_locks().await?);
        render_table(long_running_queries().await?);
        render_table(mandelbrot().await?);
        render_table(outliers().await?);
        render_table(records_rank(None).await?);
        render_table(table_size().await?);
        render_table(total_index_size().await?);
        render_table(total_table_size().await?);
        render_table(unused_indexes(None).await?);
        render_table(duplicate_indexes().await?);
        render_table(vacuum_stats().await?);
        render_table(buffercache_stats().await?);
        render_table(buffercache_usage().await?);
        render_table(ssl_used().await?);
        render_table(connections().await?);
        render_table(db_settings().await?);

        Ok(())
    }
}
