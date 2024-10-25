use std::{
    collections::HashMap,
    time::Duration,
    {env, fmt},
};
pub mod queries;
pub mod diagnose;

pub use queries::{
    all_locks::AllLocks,
    bloat::Bloat,
    blocking::Blocking,
    buffercache_stats::BuffercacheStats,
    buffercache_usage::BuffercacheUsage,
    cache_hit::CacheHit,
    calls::Calls,
    connections::Connections,
    db_settings::DbSettings,
    duplicate_indexes::DuplicateIndexes,
    extensions::Extensions,
    index_cache_hit::IndexCacheHit,
    index_scans::IndexScans,
    index_size::IndexSize,
    index_usage::IndexUsage,
    indexes::Indexes,
    locks::Locks,
    long_running_queries::LongRunningQueries,
    mandelbrot::Mandelbrot,
    null_indexes::NullIndexes,
    outliers::Outliers,
    records_rank::RecordsRank,
    seq_scans::SeqScans,
    shared::{get_default_schema, Query},
    ssl_used::SslUsed,
    table_cache_hit::TableCacheHit,
    table_index_scans::TableIndexScans,
    table_indexes_size::TableIndexesSize,
    table_size::TableSize,
    tables::Tables,
    total_index_size::TotalIndexSize,
    total_table_size::TotalTableSize,
    unused_indexes::UnusedIndexes,
    vacuum_stats::VacuumStats,
};
use semver::Version;
use sqlx::{postgres::PgPoolOptions, Row};

#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row as TableRow, Table};

pub fn render_table<T: Query>(items: Vec<T>) {
    let mut table = Table::new();
    table.add_row(T::headers());

    let columns_count = T::headers().len();

    for item in items {
        table.add_row(item.to_row());
    }
    table.set_titles(TableRow::new(vec![
        Cell::new(T::description().as_str()).style_spec(format!("H{}", columns_count).as_str())
    ]));
    table.printstd();
}

pub fn render_diagnose_result(items: Vec<CheckResult>)
{
    let mut table = Table::new();

    let mut cell = Cell::new("Diagnose Results").style_spec("H3");
    cell.align(prettytable::format::Alignment::CENTER);
    table.set_titles(TableRow::new(vec![cell]));

    table.add_row(TableRow::new(vec![
        Cell::new("Status"),
        Cell::new("Check Name"),
        Cell::new("Message"),
    ]));

    for item in items {
        table.add_row(TableRow::new(vec![
            Cell::new(if item.ok { "OK" } else { "FAIL" }),
            Cell::new(item.check_name.as_str()),
            Cell::new(item.message.as_str()),
        ]));
    }

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

pub async fn diagnose() -> Result<Vec<CheckResult>, PgExtrasError> {
    diagnose::Diagnose::run().await
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PgExtrasError {
    MissingConfigVars(),
    DbConnectionError(String),
    Unknown(String),
}

impl fmt::Display for PgExtrasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::MissingConfigVars() => {
                "Both $DATABASE_URL and $PG_EXTRAS_DATABASE_URL are not set."
            }
            Self::DbConnectionError(e) => &format!("Cannot connect to database: '{}'", e),
            Self::Unknown(e) => &format!("Unknown pg-extras error: '{}'", e),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for PgExtrasError {}

use lazy_static::lazy_static;
use crate::diagnose::CheckResult;

lazy_static! {
    pub static ref NEW_PG_STAT_STATEMENTS: Version = Version::parse("1.8.0").unwrap();
    pub static ref PG_STAT_STATEMENTS_17: Version = semver::Version::parse("1.11.0").unwrap();
}

#[derive(Debug)]
pub enum PgStatsVersion {
    Legacy,
    Standard,
    Pg17,
}

async fn get_rows<T: Query>(
    params: Option<HashMap<String, String>>,
) -> Result<Vec<T>, PgExtrasError> {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(db_url()?.as_str())
        .await
    {
        Ok(pool) => pool,
        Err(e) => return Err(PgExtrasError::DbConnectionError(format!("{}", e))),
    };

    let pg_statements_query =
        "select installed_version from pg_available_extensions where name='pg_stat_statements'";

    let pg_statements_version = match sqlx::query(pg_statements_query).fetch_one(&pool).await {
        Ok(row) => row
            .try_get::<String, _>("installed_version")
            .unwrap_or_default(),
        Err(_) => "".to_string(),
    };

    let default_version = NEW_PG_STAT_STATEMENTS.clone();
    let pg_statements_version = format!("{}.0", pg_statements_version);
    let pg_statements_version =
        Version::parse(&pg_statements_version).unwrap_or(default_version.clone());

    let pg_statements_version = if pg_statements_version < default_version {
        PgStatsVersion::Legacy
    } else if pg_statements_version >= *PG_STAT_STATEMENTS_17 {
        PgStatsVersion::Pg17
    } else {
        PgStatsVersion::Standard
    };

    let mut query = T::read_file(Some(pg_statements_version));

    if let Some(params) = params {
        for (key, value) in &params {
            query = query.replace(&format!("%{{{}}}", key), value.as_str());
        }
    }

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
        let port = match env::var("PG_VERSION").expect("PG_VERSION not set").as_str() {
            "12" => "5432",
            "13" => "5433",
            "14" => "5434",
            "15" => "5435",
            "16" => "5436",
            "17" => "5437",
            _ => "5432",
        };

        env::set_var(
            "PG_EXTRAS_DATABASE_URL",
            format!(
                "postgres://postgres:secret@localhost:{}/pg-extras-rs-test",
                port
            ),
        );

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

    #[test]
    fn normal_types() {
        fn is_normal<T: Sized + Send + Sync + Unpin>() {}

        is_normal::<NullIndexes>();
        is_normal::<Bloat>();
        is_normal::<Blocking>();
        is_normal::<Calls>();
        is_normal::<Extensions>();
        is_normal::<TableCacheHit>();
        is_normal::<Tables>();
        is_normal::<IndexCacheHit>();
        is_normal::<Indexes>();
        is_normal::<IndexSize>();
        is_normal::<IndexUsage>();
        is_normal::<IndexScans>();
        is_normal::<NullIndexes>();
        is_normal::<Locks>();
        is_normal::<AllLocks>();
        is_normal::<LongRunningQueries>();
        is_normal::<Mandelbrot>();
        is_normal::<Outliers>();
        is_normal::<RecordsRank>();
        is_normal::<SeqScans>();
        is_normal::<TableIndexScans>();
        is_normal::<TableIndexesSize>();
        is_normal::<TableSize>();
        is_normal::<TotalIndexSize>();
        is_normal::<TotalTableSize>();
        is_normal::<UnusedIndexes>();
        is_normal::<DuplicateIndexes>();
        is_normal::<VacuumStats>();
        is_normal::<DuplicateIndexes>();
        is_normal::<BuffercacheStats>();
        is_normal::<SslUsed>();
        is_normal::<Connections>();
        is_normal::<CacheHit>();
        is_normal::<DbSettings>();
        is_normal::<PgExtrasError>();
    }
}
