use std::{
    collections::HashMap,
    time::Duration,
    {env, fmt},
};
pub mod diagnose;
pub mod queries;

#[cfg(feature = "web")]
pub mod web;

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
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};

#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row as TableRow, Table};

/// Renders a table to stdout for any type that implements the Query trait.
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

/// Returns table and index bloat in your database ordered by most wasteful.
pub async fn bloat(pool: &Pool<Postgres>) -> Result<Vec<Bloat>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Lists queries that are blocking other queries.
pub async fn blocking(
    limit: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<Blocking>, PgExtrasError> {
    get_rows(Some(limit_params(limit)), pool).await
}

/// Creates a new connection pool to PostgreSQL.
///
/// Uses either PG_EXTRAS_DATABASE_URL or DATABASE_URL environment variable.
pub async fn pg_pool() -> Result<Pool<Postgres>, PgExtrasError> {
    match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(db_url()?.as_str())
        .await
    {
        Ok(pool) => Ok(pool),
        Err(e) => Err(PgExtrasError::DbConnectionError(format!("{}", e))),
    }
}

/// Returns statistics about query calls in the database.
pub async fn calls(
    limit: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<Calls>, PgExtrasError> {
    get_rows(Some(limit_params(limit)), pool).await
}

/// Lists all installed PostgreSQL extensions.
pub async fn extensions(pool: &Pool<Postgres>) -> Result<Vec<Extensions>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows cache hit rates for tables.
pub async fn table_cache_hit(pool: &Pool<Postgres>) -> Result<Vec<TableCacheHit>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Lists all tables in the database with their basic information.
pub async fn tables(
    schema: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<Tables>, PgExtrasError> {
    get_rows(Some(schema_params(schema)), pool).await
}

/// Shows index cache hit rates.
pub async fn index_cache_hit(
    schema: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<IndexCacheHit>, PgExtrasError> {
    get_rows(Some(schema_params(schema)), pool).await
}

/// Lists all indexes in the database.
pub async fn indexes(pool: &Pool<Postgres>) -> Result<Vec<Indexes>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows the size of all indexes, ordered by size.
pub async fn index_size(pool: &Pool<Postgres>) -> Result<Vec<IndexSize>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows statistics about index usage.
pub async fn index_usage(
    schema: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<IndexUsage>, PgExtrasError> {
    get_rows(Some(schema_params(schema)), pool).await
}

/// Shows statistics about index scans.
pub async fn index_scans(
    schema: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<IndexScans>, PgExtrasError> {
    get_rows(Some(schema_params(schema)), pool).await
}

/// Shows indexes that contain mostly NULL values.
pub async fn null_indexes(
    min_relation_size_mb: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<NullIndexes>, PgExtrasError> {
    let min_relation_size_mb = min_relation_size_mb.unwrap_or("10".to_string());

    let params: HashMap<String, String> = [(
        "min_relation_size_mb".to_string(),
        min_relation_size_mb.to_string(),
    )]
    .iter()
    .cloned()
    .collect();
    get_rows(Some(params), pool).await
}

/// Shows information about locks in the database.
pub async fn locks(pool: &Pool<Postgres>) -> Result<Vec<Locks>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows detailed information about all locks in the database.
pub async fn all_locks(pool: &Pool<Postgres>) -> Result<Vec<AllLocks>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Lists currently running queries that have been running for a long time.
pub async fn long_running_queries(
    pool: &Pool<Postgres>,
) -> Result<Vec<LongRunningQueries>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Generates a Mandelbrot set as a test query.
pub async fn mandelbrot(pool: &Pool<Postgres>) -> Result<Vec<Mandelbrot>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows queries with the longest execution time in aggregate.
pub async fn outliers(pool: &Pool<Postgres>) -> Result<Vec<Outliers>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows estimated number of rows in each table, ordered by estimated count.
pub async fn records_rank(
    schema: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<RecordsRank>, PgExtrasError> {
    get_rows(Some(schema_params(schema)), pool).await
}

/// Shows statistics about sequential scans performed on tables.
pub async fn seq_scans(
    schema: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<SeqScans>, PgExtrasError> {
    get_rows(Some(schema_params(schema)), pool).await
}

/// Shows statistics about index scans performed on tables.
pub async fn table_index_scans(
    schema: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<TableIndexScans>, PgExtrasError> {
    get_rows(Some(schema_params(schema)), pool).await
}

/// Shows total size of all indexes for each table.
pub async fn table_indexes_size(
    schema: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<TableIndexesSize>, PgExtrasError> {
    get_rows(Some(schema_params(schema)), pool).await
}

/// Shows disk space used by each table, excluding indexes.
pub async fn table_size(pool: &Pool<Postgres>) -> Result<Vec<TableSize>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows total size of all indexes in the database.
pub async fn total_index_size(pool: &Pool<Postgres>) -> Result<Vec<TotalIndexSize>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows total disk space used by tables and indexes.
pub async fn total_table_size(pool: &Pool<Postgres>) -> Result<Vec<TotalTableSize>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Lists indexes that haven't been used or are rarely used.
pub async fn unused_indexes(
    schema: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<UnusedIndexes>, PgExtrasError> {
    get_rows(Some(schema_params(schema)), pool).await
}

/// Shows indexes that have identical definitions but different names.
pub async fn duplicate_indexes(
    pool: &Pool<Postgres>,
) -> Result<Vec<DuplicateIndexes>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows statistics about VACUUM and ANALYZE operations.
pub async fn vacuum_stats(pool: &Pool<Postgres>) -> Result<Vec<VacuumStats>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows statistics about shared buffer cache usage.
pub async fn buffercache_stats(
    pool: &Pool<Postgres>,
) -> Result<Vec<BuffercacheStats>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows distribution of buffer cache usage by database objects.
pub async fn buffercache_usage(
    pool: &Pool<Postgres>,
) -> Result<Vec<BuffercacheUsage>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows whether SSL is being used for current connections.
pub async fn ssl_used(pool: &Pool<Postgres>) -> Result<Vec<SslUsed>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows information about current database connections and their states.
pub async fn connections(pool: &Pool<Postgres>) -> Result<Vec<Connections>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Shows cache hit rates for both tables and indexes.
pub async fn cache_hit(
    schema: Option<String>,
    pool: &Pool<Postgres>,
) -> Result<Vec<CacheHit>, PgExtrasError> {
    get_rows(Some(schema_params(schema)), pool).await
}

/// Shows current values of important PostgreSQL settings.
pub async fn db_settings(pool: &Pool<Postgres>) -> Result<Vec<DbSettings>, PgExtrasError> {
    get_rows(None, pool).await
}

/// Runs a comprehensive set of diagnostic checks on the database.
pub async fn diagnose(pool: &Pool<Postgres>) -> Result<Vec<CheckResult>, PgExtrasError> {
    run_diagnose(pool).await
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PgExtrasError {
    MissingConfigVars(),
    DbConnectionError(String),
    Other(String),
}

impl fmt::Display for PgExtrasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::MissingConfigVars() => {
                "Both $DATABASE_URL and $PG_EXTRAS_DATABASE_URL are not set."
            }
            Self::DbConnectionError(e) => &format!("Cannot connect to database: '{}'", e),
            Self::Other(e) => &e.to_string(),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for PgExtrasError {}

use crate::diagnose::run::{run_diagnose, CheckResult};
use lazy_static::lazy_static;

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
    pool: &Pool<Postgres>,
) -> Result<Vec<T>, PgExtrasError> {
    let pg_statements_query =
        "select installed_version from pg_available_extensions where name='pg_stat_statements'";

    let pg_statements_version = match sqlx::query(pg_statements_query).fetch_one(pool).await {
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

    Ok(match sqlx::query(&query).fetch_all(pool).await {
        Ok(rows) => rows.iter().map(T::new).collect(),
        Err(e) => return Err(PgExtrasError::Other(format!("{}", e))),
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
    use crate::diagnose::report::render_diagnose_report;

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

        let pool = pg_pool().await?;

        render_table(cache_hit(None, &pool).await?);
        render_table(bloat(&pool).await?);
        render_table(blocking(None, &pool).await?);
        render_table(calls(None, &pool).await?);
        render_table(extensions(&pool).await?);
        render_table(table_cache_hit(&pool).await?);
        render_table(seq_scans(None, &pool).await?);
        render_table(table_index_scans(None, &pool).await?);
        render_table(table_indexes_size(None, &pool).await?);
        render_table(tables(None, &pool).await?);
        render_table(index_cache_hit(None, &pool).await?);
        render_table(indexes(&pool).await?);
        render_table(index_size(&pool).await?);
        render_table(index_usage(None, &pool).await?);
        render_table(index_scans(None, &pool).await?);
        render_table(null_indexes(None, &pool).await?);
        render_table(locks(&pool).await?);
        render_table(all_locks(&pool).await?);
        render_table(long_running_queries(&pool).await?);
        render_table(mandelbrot(&pool).await?);
        render_table(outliers(&pool).await?);
        render_table(records_rank(None, &pool).await?);
        render_table(table_size(&pool).await?);
        render_table(total_index_size(&pool).await?);
        render_table(total_table_size(&pool).await?);
        render_table(unused_indexes(None, &pool).await?);
        render_table(duplicate_indexes(&pool).await?);
        render_table(vacuum_stats(&pool).await?);
        render_table(buffercache_stats(&pool).await?);
        render_table(buffercache_usage(&pool).await?);
        render_table(ssl_used(&pool).await?);
        render_table(connections(&pool).await?);
        render_table(db_settings(&pool).await?);
        render_diagnose_report(diagnose(&pool).await?);

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
