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
pub use structs::db_settings::DbSettings;
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
    let query = Bloat::read_file();
    get_rows(query).await
}

pub async fn blocking(limit: Option<String>) -> Result<Vec<Blocking>, PgExtrasError> {
    let limit = limit.unwrap_or("10".to_string());
    let query = Blocking::read_file().replace("%{limit}", limit.as_str());
    get_rows(&query).await
}

pub async fn calls(limit: Option<String>) -> Result<Vec<Calls>, PgExtrasError> {
    let limit = limit.unwrap_or("10".to_string());
    let query = Calls::read_file().replace("%{limit}", limit.as_str());
    get_rows(&query).await
}

pub async fn extensions() -> Result<Vec<Extensions>, PgExtrasError> {
    let query = Extensions::read_file();
    get_rows(query).await
}

pub async fn table_cache_hit() -> Result<Vec<TableCacheHit>, PgExtrasError> {
    let query = TableCacheHit::read_file();
    get_rows(query).await
}

pub async fn tables(schema: Option<String>) -> Result<Vec<Tables>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Tables::read_file().replace("%{schema}", &schema_name);
    get_rows(&query).await
}

pub async fn index_cache_hit(schema: Option<String>) -> Result<Vec<IndexCacheHit>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = IndexCacheHit::read_file().replace("%{schema}", &schema_name);
    get_rows(&query).await
}

pub async fn indexes() -> Result<Vec<Indexes>, PgExtrasError> {
    let query = Indexes::read_file();
    get_rows(query).await
}

pub async fn index_size() -> Result<Vec<IndexSize>, PgExtrasError> {
    let query = IndexSize::read_file();
    get_rows(query).await
}

pub async fn index_usage(schema: Option<String>) -> Result<Vec<IndexUsage>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = IndexUsage::read_file().replace("%{schema}", &schema_name);
    get_rows(&query).await
}

pub async fn index_scans(schema: Option<String>) -> Result<Vec<IndexScans>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = IndexScans::read_file().replace("%{schema}", &schema_name);
    get_rows(&query).await
}

pub async fn null_indexes(
    min_relation_size_mb: Option<String>,
) -> Result<Vec<NullIndexes>, PgExtrasError> {
    let min_relation_size_mb = min_relation_size_mb.unwrap_or("0".to_string());
    let query = NullIndexes::read_file().replace("%{min_relation_size_mb}", &min_relation_size_mb);
    get_rows(&query).await
}

pub async fn locks() -> Result<Vec<Locks>, PgExtrasError> {
    let query = Locks::read_file();
    get_rows(query).await
}

pub async fn all_locks() -> Result<Vec<AllLocks>, PgExtrasError> {
    let query = AllLocks::read_file();
    get_rows(query).await
}

pub async fn long_running_queries() -> Result<Vec<LongRunningQueries>, PgExtrasError> {
    let query = LongRunningQueries::read_file();
    get_rows(query).await
}

pub async fn mandelbrot() -> Result<Vec<Mandelbrot>, PgExtrasError> {
    let query = Mandelbrot::read_file();
    get_rows(query).await
}

pub async fn outliers() -> Result<Vec<Outliers>, PgExtrasError> {
    let query = Outliers::read_file();
    get_rows(query).await
}

pub async fn records_rank(schema: Option<String>) -> Result<Vec<RecordsRank>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = RecordsRank::read_file().replace("%{schema}", schema_name.as_str());
    get_rows(&query).await
}

pub async fn seq_scans(schema: Option<String>) -> Result<Vec<SeqScans>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = SeqScans::read_file().replace("%{schema}", schema_name.as_str());
    get_rows(&query).await
}

pub async fn table_index_scans(
    schema: Option<String>,
) -> Result<Vec<TableIndexScans>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = TableIndexScans::read_file().replace("%{schema}", schema_name.as_str());
    get_rows(&query).await
}

pub async fn table_indexes_size(
    schema: Option<String>,
) -> Result<Vec<TableIndexesSize>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = TableIndexesSize::read_file().replace("%{schema}", schema_name.as_str());
    get_rows(&query).await
}

pub async fn table_size() -> Result<Vec<TableSize>, PgExtrasError> {
    let query = TableSize::read_file();
    get_rows(query).await
}

pub async fn total_index_size() -> Result<Vec<TotalIndexSize>, PgExtrasError> {
    let query = TotalIndexSize::read_file();
    get_rows(query).await
}

pub async fn total_table_size() -> Result<Vec<TotalTableSize>, PgExtrasError> {
    let query = TotalTableSize::read_file();
    get_rows(query).await
}

pub async fn unused_indexes(schema: Option<String>) -> Result<Vec<UnusedIndexes>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = UnusedIndexes::read_file().replace("%{schema}", schema_name.as_str());
    get_rows(&query).await
}

pub async fn duplicate_indexes() -> Result<Vec<DuplicateIndexes>, PgExtrasError> {
    let query = DuplicateIndexes::read_file();
    get_rows(query).await
}

pub async fn vacuum_stats() -> Result<Vec<VacuumStats>, PgExtrasError> {
    let query = VacuumStats::read_file();
    get_rows(query).await
}

pub async fn buffercache_stats() -> Result<Vec<BuffercacheStats>, PgExtrasError> {
    let query = BuffercacheStats::read_file();
    get_rows(query).await
}

pub async fn buffercache_usage() -> Result<Vec<BuffercacheUsage>, PgExtrasError> {
    let query = BuffercacheUsage::read_file();
    get_rows(query).await
}

pub async fn ssl_used() -> Result<Vec<SslUsed>, PgExtrasError> {
    let query = SslUsed::read_file();
    get_rows(query).await
}

pub async fn connections() -> Result<Vec<Connections>, PgExtrasError> {
    let query = Connections::read_file();
    get_rows(query).await
}

pub async fn cache_hit(schema: Option<String>) -> Result<Vec<CacheHit>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = CacheHit::read_file().replace("%{schema}", schema_name.as_str());
    get_rows(&query).await
}

pub async fn db_settings() -> Result<Vec<DbSettings>, PgExtrasError> {
    let query = DbSettings::read_file();
    get_rows(query).await
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
