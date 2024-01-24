use std::env;
pub mod structs;
use sqlx::postgres::PgPoolOptions;
pub use structs::all_locks::AllLocks;
pub use structs::bloat::Bloat;
pub use structs::blocking::Blocking;
pub use structs::buffercache_stats::BuffercacheStats;
pub use structs::buffercache_usage::BuffercacheUsage;
pub use structs::cache_hit::CacheHit;
pub use structs::calls::Calls;
pub use structs::connections::Connections;
pub use structs::db_settings::DbSetting;
pub use structs::duplicate_indexes::DuplicateIndexes;
pub use structs::extensions::Extensions;
pub use structs::index_cache_hit::IndexCacheHit;
pub use structs::index_scans::IndexScans;
pub use structs::index_size::IndexSize;
pub use structs::index_usage::IndexUsage;
pub use structs::indexes::Indexes;
pub use structs::locks::Locks;
pub use structs::long_running_queries::LongRunningQueries;
pub use structs::mandelbrot::Mandelbrot;
pub use structs::null_indexes::NullIndexes;
pub use structs::outliers::Outliers;
pub use structs::records_rank::RecordsRank;
pub use structs::seq_scans::SeqScans;
pub use structs::shared::{get_default_schema, Tabular};
pub use structs::ssl_used::SslUsed;
pub use structs::table_cache_hit::TableCacheHit;
pub use structs::table_index_scans::TableIndexScans;
pub use structs::table_indexes_size::TableIndexesSize;
pub use structs::table_size::TableSize;
pub use structs::tables::Tables;
pub use structs::total_index_size::TotalIndexSize;
pub use structs::total_table_size::TotalTableSize;
pub use structs::unused_indexes::UnusedIndexes;
pub use structs::vacuum_stats::VacuumStats;
use thiserror::Error;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

pub fn render_table<T: Tabular>(items: Vec<T>) {
    let mut table = Table::new();
    table.add_row(T::headers());

    for item in items {
        table.add_row(item.to_row());
    }
    table.printstd();
}

pub async fn bloat() -> Result<Vec<Bloat>, PgExtrasError> {
    let query = Query::read_file(Query::Bloat);
    get_rows::<Bloat>(query).await
}

pub async fn blocking(limit: Option<String>) -> Result<Vec<Blocking>, PgExtrasError> {
    let limit = limit.unwrap_or("10".to_string());
    let query = Query::read_file(Query::Blocking).replace("%{limit}", limit.as_str());
    get_rows::<Blocking>(&query).await
}

pub async fn calls(limit: Option<String>) -> Result<Vec<Calls>, PgExtrasError> {
    let limit = limit.unwrap_or("10".to_string());
    let query = Query::read_file(Query::Calls).replace("%{limit}", limit.as_str());
    get_rows::<Calls>(&query).await
}

pub async fn extensions() -> Result<Vec<Extensions>, PgExtrasError> {
    let query = Query::read_file(Query::Extensions);
    get_rows::<Extensions>(query).await
}

pub async fn table_cache_hit() -> Result<Vec<TableCacheHit>, PgExtrasError> {
    let query = Query::read_file(Query::TableCacheHit);
    get_rows::<TableCacheHit>(query).await
}

pub async fn tables(schema: Option<String>) -> Result<Vec<Tables>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::Tables).replace("%{schema}", &schema_name);
    get_rows::<Tables>(&query).await
}

pub async fn index_cache_hit(schema: Option<String>) -> Result<Vec<IndexCacheHit>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::IndexCacheHit).replace("%{schema}", &schema_name);
    get_rows::<IndexCacheHit>(&query).await
}

pub async fn indexes() -> Result<Vec<Indexes>, PgExtrasError> {
    let query = Query::read_file(Query::Indexes);
    get_rows::<Indexes>(query).await
}

pub async fn index_size() -> Result<Vec<IndexSize>, PgExtrasError> {
    let query = Query::read_file(Query::IndexSize);
    get_rows::<IndexSize>(query).await
}

pub async fn index_usage(schema: Option<String>) -> Result<Vec<IndexUsage>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::IndexUsage).replace("%{schema}", &schema_name);
    get_rows::<IndexUsage>(&query).await
}

pub async fn index_scans(schema: Option<String>) -> Result<Vec<IndexScans>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::IndexScans).replace("%{schema}", &schema_name);
    get_rows::<IndexScans>(&query).await
}

pub async fn null_indexes(
    min_relation_size_mb: Option<String>,
) -> Result<Vec<NullIndexes>, PgExtrasError> {
    let min_relation_size_mb = min_relation_size_mb.unwrap_or("0".to_string());
    let query = Query::read_file(Query::NullIndexes)
        .replace("%{min_relation_size_mb}", &min_relation_size_mb);
    get_rows::<NullIndexes>(&query).await
}

pub async fn locks() -> Result<Vec<Locks>, PgExtrasError> {
    let query = Query::read_file(Query::Locks);
    get_rows::<Locks>(query).await
}

pub async fn all_locks() -> Result<Vec<AllLocks>, PgExtrasError> {
    let query = Query::read_file(Query::AllLocks);
    get_rows::<AllLocks>(query).await
}

pub async fn long_running_queries() -> Result<Vec<LongRunningQueries>, PgExtrasError> {
    let query = Query::read_file(Query::LongRunningQueries);
    get_rows::<LongRunningQueries>(query).await
}

pub async fn mandelbrot() -> Result<Vec<Mandelbrot>, PgExtrasError> {
    let query = Query::read_file(Query::Mandelbrot);
    get_rows::<Mandelbrot>(query).await
}

pub async fn outliers() -> Result<Vec<Outliers>, PgExtrasError> {
    let query = Query::read_file(Query::Outliers);
    get_rows::<Outliers>(query).await
}

pub async fn records_rank(schema: Option<String>) -> Result<Vec<RecordsRank>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::RecordsRank).replace("%{schema}", schema_name.as_str());
    get_rows::<RecordsRank>(&query).await
}

pub async fn seq_scans(schema: Option<String>) -> Result<Vec<SeqScans>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::SeqScans).replace("%{schema}", schema_name.as_str());
    get_rows::<SeqScans>(&query).await
}

pub async fn table_index_scans(
    schema: Option<String>,
) -> Result<Vec<TableIndexScans>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::TableIndexScans).replace("%{schema}", schema_name.as_str());
    get_rows::<TableIndexScans>(&query).await
}

pub async fn table_indexes_size(
    schema: Option<String>,
) -> Result<Vec<TableIndexesSize>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query =
        Query::read_file(Query::TableIndexesSize).replace("%{schema}", schema_name.as_str());
    get_rows::<TableIndexesSize>(&query).await
}

pub async fn table_size() -> Result<Vec<TableSize>, PgExtrasError> {
    let query = Query::read_file(Query::TableSize);
    get_rows::<TableSize>(query).await
}

pub async fn total_index_size() -> Result<Vec<TotalIndexSize>, PgExtrasError> {
    let query = Query::read_file(Query::TotalIndexSize);
    get_rows::<TotalIndexSize>(query).await
}

pub async fn total_table_size() -> Result<Vec<TotalTableSize>, PgExtrasError> {
    let query = Query::read_file(Query::TotalTableSize);
    get_rows::<TotalTableSize>(query).await
}

pub async fn unused_indexes(schema: Option<String>) -> Result<Vec<UnusedIndexes>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::UnusedIndexes).replace("%{schema}", schema_name.as_str());
    get_rows::<UnusedIndexes>(&query).await
}

pub async fn duplicate_indexes() -> Result<Vec<DuplicateIndexes>, PgExtrasError> {
    let query = Query::read_file(Query::DuplicateIndexes);
    get_rows::<DuplicateIndexes>(query).await
}

pub async fn vacuum_stats() -> Result<Vec<VacuumStats>, PgExtrasError> {
    let query = Query::read_file(Query::VacuumStats);
    get_rows::<VacuumStats>(query).await
}

pub async fn buffercache_stats() -> Result<Vec<BuffercacheStats>, PgExtrasError> {
    let query = Query::read_file(Query::BuffercacheStats);
    get_rows::<BuffercacheStats>(query).await
}

pub async fn buffercache_usage() -> Result<Vec<BuffercacheUsage>, PgExtrasError> {
    let query = Query::read_file(Query::BuffercacheUsage);
    get_rows::<BuffercacheUsage>(query).await
}

pub async fn ssl_used() -> Result<Vec<SslUsed>, PgExtrasError> {
    let query = Query::read_file(Query::SslUsed);
    get_rows::<SslUsed>(query).await
}

pub async fn connections() -> Result<Vec<Connections>, PgExtrasError> {
    let query = Query::read_file(Query::Connections);
    get_rows::<Connections>(query).await
}

pub async fn cache_hit(schema: Option<String>) -> Result<Vec<CacheHit>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::CacheHit).replace("%{schema}", schema_name.as_str());
    get_rows::<CacheHit>(&query).await
}

pub async fn db_settings() -> Result<Vec<DbSetting>, PgExtrasError> {
    let query = Query::read_file(Query::DbSettings);
    get_rows::<DbSetting>(query).await
}

enum Query {
    CacheHit,
    Bloat,
    Blocking,
    Calls,
    Extensions,
    TableCacheHit,
    Tables,
    IndexCacheHit,
    DbSettings,
    Indexes,
    IndexSize,
    IndexUsage,
    IndexScans,
    NullIndexes,
    Locks,
    AllLocks,
    LongRunningQueries,
    Mandelbrot,
    Outliers,
    RecordsRank,
    SeqScans,
    TableIndexScans,
    TableIndexesSize,
    TableSize,
    TotalIndexSize,
    TotalTableSize,
    UnusedIndexes,
    DuplicateIndexes,
    VacuumStats,
    BuffercacheStats,
    BuffercacheUsage,
    SslUsed,
    Connections,
}

impl Query {
    pub fn read_file(query: Query) -> &'static str {
        match query {
            Query::CacheHit => include_str!("queries/cache_hit.sql"),
            Query::DbSettings => include_str!("queries/db_settings.sql"),
            Query::BuffercacheStats => include_str!("queries/buffercache_stats.sql"),
            Query::BuffercacheUsage => include_str!("queries/buffercache_usage.sql"),
            Query::SslUsed => include_str!("queries/ssl_used.sql"),
            Query::Connections => include_str!("queries/connections.sql"),
            Query::Bloat => include_str!("queries/bloat.sql"),
            Query::Blocking => include_str!("queries/blocking.sql"),
            Query::Calls => include_str!("queries/calls.sql"),
            Query::Extensions => include_str!("queries/extensions.sql"),
            Query::TableCacheHit => include_str!("queries/table_cache_hit.sql"),
            Query::Tables => include_str!("queries/tables.sql"),
            Query::IndexCacheHit => include_str!("queries/index_cache_hit.sql"),
            Query::Indexes => include_str!("queries/indexes.sql"),
            Query::IndexSize => include_str!("queries/index_size.sql"),
            Query::IndexUsage => include_str!("queries/index_usage.sql"),
            Query::IndexScans => include_str!("queries/index_scans.sql"),
            Query::NullIndexes => include_str!("queries/null_indexes.sql"),
            Query::Locks => include_str!("queries/locks.sql"),
            Query::AllLocks => include_str!("queries/all_locks.sql"),
            Query::LongRunningQueries => include_str!("queries/long_running_queries.sql"),
            Query::Mandelbrot => include_str!("queries/mandelbrot.sql"),
            Query::Outliers => include_str!("queries/outliers.sql"),
            Query::RecordsRank => include_str!("queries/records_rank.sql"),
            Query::SeqScans => include_str!("queries/seq_scans.sql"),
            Query::TableIndexScans => include_str!("queries/table_index_scans.sql"),
            Query::TableIndexesSize => include_str!("queries/table_indexes_size.sql"),
            Query::TableSize => include_str!("queries/table_size.sql"),
            Query::TotalIndexSize => include_str!("queries/total_index_size.sql"),
            Query::TotalTableSize => include_str!("queries/total_table_size.sql"),
            Query::UnusedIndexes => include_str!("queries/unused_indexes.sql"),
            Query::DuplicateIndexes => include_str!("queries/duplicate_indexes.sql"),
            Query::VacuumStats => include_str!("queries/vacuum_stats.sql"),
        }
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PgExtrasError {
    #[error("Both $DATABASE_URL and $PG_EXTRAS_DATABASE_URL are not set")]
    MissingConfigVars(),
    #[error("Cannot connect to database")]
    DbConnectionError(String),
    #[error("Unknown pg-extras error")]
    Unknown(String),
}

async fn get_rows<T: Tabular>(query: &str) -> Result<Vec<T>, PgExtrasError> {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url()?.as_str())
        .await
    {
        Ok(pool) => pool,
        Err(e) => return Err(PgExtrasError::DbConnectionError(format!("{}", e))),
    };

    Ok(match sqlx::query(query).fetch_all(&pool).await {
        Ok(rows) => rows.iter().map(T::new).collect(),
        Err(e) => return Err(PgExtrasError::Unknown(format!("{}", e))),
    })
}

fn db_url() -> Result<String, PgExtrasError> {
    env::var("PG_EXTRAS_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .map_err(|_| PgExtrasError::MissingConfigVars())
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
