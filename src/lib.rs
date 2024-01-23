use postgres::{Client, NoTls, Row};
use std::env;
pub mod structs;
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

pub fn bloat() -> Result<Vec<Bloat>, PgExtrasError> {
    let query = Query::read_file(Query::Bloat);
    Ok(get_rows(query)?.iter().map(Bloat::new).collect())
}

pub fn blocking(limit: Option<String>) -> Result<Vec<Blocking>, PgExtrasError> {
    let limit = limit.unwrap_or("10".to_string());
    let query = Query::read_file(Query::Blocking).replace("%{limit}", limit.as_str());
    Ok(get_rows(&query)?.iter().map(Blocking::new).collect())
}

pub fn calls(limit: Option<String>) -> Result<Vec<Calls>, PgExtrasError> {
    let limit = limit.unwrap_or("10".to_string());
    let query = Query::read_file(Query::Calls).replace("%{limit}", limit.as_str());
    Ok(get_rows(&query)?.iter().map(Calls::new).collect())
}

pub fn extensions() -> Result<Vec<Extensions>, PgExtrasError> {
    let query = Query::read_file(Query::Extensions);
    Ok(get_rows(query)?.iter().map(Extensions::new).collect())
}

pub fn table_cache_hit() -> Result<Vec<TableCacheHit>, PgExtrasError> {
    let query = Query::read_file(Query::TableCacheHit);
    Ok(get_rows(query)?.iter().map(TableCacheHit::new).collect())
}

pub fn tables(schema: Option<String>) -> Result<Vec<Tables>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::Tables).replace("%{schema}", &schema_name);
    Ok(get_rows(&query)?.iter().map(Tables::new).collect())
}

pub fn index_cache_hit(schema: Option<String>) -> Result<Vec<IndexCacheHit>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::IndexCacheHit).replace("%{schema}", &schema_name);
    Ok(get_rows(&query)?.iter().map(IndexCacheHit::new).collect())
}

pub fn indexes() -> Result<Vec<Indexes>, PgExtrasError> {
    let query = Query::read_file(Query::Indexes);
    Ok(get_rows(query)?.iter().map(Indexes::new).collect())
}

pub fn index_size() -> Result<Vec<IndexSize>, PgExtrasError> {
    let query = Query::read_file(Query::IndexSize);
    Ok(get_rows(query)?.iter().map(IndexSize::new).collect())
}

pub fn index_usage(schema: Option<String>) -> Result<Vec<IndexUsage>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::IndexUsage).replace("%{schema}", &schema_name);
    Ok(get_rows(&query)?.iter().map(IndexUsage::new).collect())
}

pub fn index_scans(schema: Option<String>) -> Result<Vec<IndexScans>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::IndexScans).replace("%{schema}", &schema_name);
    Ok(get_rows(&query)?.iter().map(IndexScans::new).collect())
}

pub fn null_indexes(
    min_relation_size_mb: Option<String>,
) -> Result<Vec<NullIndexes>, PgExtrasError> {
    let min_relation_size_mb = min_relation_size_mb.unwrap_or("0".to_string());
    let query = Query::read_file(Query::NullIndexes)
        .replace("%{min_relation_size_mb}", &min_relation_size_mb);
    Ok(get_rows(&query)?.iter().map(NullIndexes::new).collect())
}

pub fn locks() -> Result<Vec<Locks>, PgExtrasError> {
    let query = Query::read_file(Query::Locks);
    Ok(get_rows(query)?.iter().map(Locks::new).collect())
}

pub fn all_locks() -> Result<Vec<AllLocks>, PgExtrasError> {
    let query = Query::read_file(Query::AllLocks);
    Ok(get_rows(query)?.iter().map(AllLocks::new).collect())
}

pub fn long_running_queries() -> Result<Vec<LongRunningQueries>, PgExtrasError> {
    let query = Query::read_file(Query::LongRunningQueries);
    Ok(get_rows(query)?
        .iter()
        .map(LongRunningQueries::new)
        .collect())
}

pub fn mandelbrot() -> Result<Vec<Mandelbrot>, PgExtrasError> {
    let query = Query::read_file(Query::Mandelbrot);
    Ok(get_rows(query)?.iter().map(Mandelbrot::new).collect())
}

pub fn outliers() -> Result<Vec<Outliers>, PgExtrasError> {
    let query = Query::read_file(Query::Outliers);
    Ok(get_rows(query)?.iter().map(Outliers::new).collect())
}

pub fn records_rank(schema: Option<String>) -> Result<Vec<RecordsRank>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::RecordsRank).replace("%{schema}", schema_name.as_str());
    Ok(get_rows(&query)?.iter().map(RecordsRank::new).collect())
}

pub fn seq_scans(schema: Option<String>) -> Result<Vec<SeqScans>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::SeqScans).replace("%{schema}", schema_name.as_str());
    Ok(get_rows(&query)?.iter().map(SeqScans::new).collect())
}

pub fn table_index_scans(schema: Option<String>) -> Result<Vec<TableIndexScans>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::TableIndexScans).replace("%{schema}", schema_name.as_str());
    Ok(get_rows(&query)?.iter().map(TableIndexScans::new).collect())
}

pub fn table_indexes_size(schema: Option<String>) -> Result<Vec<TableIndexesSize>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query =
        Query::read_file(Query::TableIndexesSize).replace("%{schema}", schema_name.as_str());
    Ok(get_rows(&query)?
        .iter()
        .map(TableIndexesSize::new)
        .collect())
}

pub fn table_size() -> Result<Vec<TableSize>, PgExtrasError> {
    let query = Query::read_file(Query::TableSize);
    Ok(get_rows(query)?.iter().map(TableSize::new).collect())
}

pub fn total_index_size() -> Result<Vec<TotalIndexSize>, PgExtrasError> {
    let query = Query::read_file(Query::TotalIndexSize);
    Ok(get_rows(query)?.iter().map(TotalIndexSize::new).collect())
}

pub fn total_table_size() -> Result<Vec<TotalTableSize>, PgExtrasError> {
    let query = Query::read_file(Query::TotalTableSize);
    Ok(get_rows(query)?.iter().map(TotalTableSize::new).collect())
}

pub fn unused_indexes(schema: Option<String>) -> Result<Vec<UnusedIndexes>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::UnusedIndexes).replace("%{schema}", schema_name.as_str());
    Ok(get_rows(&query)?.iter().map(UnusedIndexes::new).collect())
}

pub fn duplicate_indexes() -> Result<Vec<DuplicateIndexes>, PgExtrasError> {
    let query = Query::read_file(Query::DuplicateIndexes);
    Ok(get_rows(query)?.iter().map(DuplicateIndexes::new).collect())
}

pub fn vacuum_stats() -> Result<Vec<VacuumStats>, PgExtrasError> {
    let query = Query::read_file(Query::VacuumStats);
    Ok(get_rows(query)?.iter().map(VacuumStats::new).collect())
}

pub fn buffercache_stats() -> Result<Vec<BuffercacheStats>, PgExtrasError> {
    let query = Query::read_file(Query::BuffercacheStats);
    Ok(get_rows(query)?.iter().map(BuffercacheStats::new).collect())
}

pub fn buffercache_usage() -> Result<Vec<BuffercacheUsage>, PgExtrasError> {
    let query = Query::read_file(Query::BuffercacheUsage);
    Ok(get_rows(query)?.iter().map(BuffercacheUsage::new).collect())
}

pub fn ssl_used() -> Result<Vec<SslUsed>, PgExtrasError> {
    let query = Query::read_file(Query::SslUsed);
    Ok(get_rows(query)?.iter().map(SslUsed::new).collect())
}

pub fn connections() -> Result<Vec<Connections>, PgExtrasError> {
    let query = Query::read_file(Query::Connections);
    Ok(get_rows(query)?.iter().map(Connections::new).collect())
}

pub fn cache_hit(schema: Option<String>) -> Result<Vec<CacheHit>, PgExtrasError> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = Query::read_file(Query::CacheHit).replace("%{schema}", schema_name.as_str());
    Ok(get_rows(&query)?.iter().map(CacheHit::new).collect())
}

pub fn db_settings() -> Result<Vec<DbSetting>, PgExtrasError> {
    let query = Query::read_file(Query::DbSettings);
    Ok(get_rows(query)?.iter().map(DbSetting::new).collect())
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
pub enum PgExtrasError {
    #[error("Both $DATABASE_URL and $PG_EXTRAS_DATABASE_URL are not set")]
    MissingConfigVars(),
    #[error("Cannot connect to database")]
    ConnectionError(),
    #[error("Unknown pg-extras error")]
    Unknown,
}

fn get_rows(query: &str) -> Result<Vec<Row>, PgExtrasError> {
    Ok(connection()?
        .query(query, &[])
        .unwrap_or_else(|_| Vec::new()))
}

fn connection() -> Result<Client, PgExtrasError> {
    let database_url =
        match env::var("PG_EXTRAS_DATABASE_URL").or_else(|_| env::var("DATABASE_URL")) {
            Ok(url) => url,
            Err(_) => return Err(PgExtrasError::MissingConfigVars()),
        };

    match Client::connect(&database_url, NoTls) {
        Ok(client) => Ok(client),
        Err(_) => Err(PgExtrasError::ConnectionError()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), PgExtrasError> {
        render_table(cache_hit(None)?);
        render_table(bloat()?);
        render_table(blocking(None)?);
        render_table(calls(None)?);
        render_table(extensions()?);
        render_table(table_cache_hit()?);
        render_table(tables(None)?);
        render_table(index_cache_hit(None)?);
        render_table(indexes()?);
        render_table(index_size()?);
        render_table(index_usage(None)?);
        render_table(index_scans(None)?);
        render_table(null_indexes(None)?);
        render_table(locks()?);
        render_table(all_locks()?);
        render_table(long_running_queries()?);
        render_table(mandelbrot()?);
        render_table(outliers()?);
        render_table(records_rank(None)?);
        render_table(seq_scans(None)?);
        render_table(table_index_scans(None)?);
        render_table(table_indexes_size(None)?);
        render_table(table_size()?);
        render_table(total_index_size()?);
        render_table(total_table_size()?);
        render_table(unused_indexes(None)?);
        render_table(duplicate_indexes()?);
        render_table(vacuum_stats()?);
        render_table(buffercache_stats()?);
        render_table(buffercache_usage()?);
        render_table(ssl_used()?);
        render_table(connections()?);
        Ok(())
    }
}
