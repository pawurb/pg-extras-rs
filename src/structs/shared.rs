use sqlx::postgres::{types::PgInterval, PgRow};
use std::env;

pub trait Tabular {
    const FILE_NAME: Query;
    fn new(row: &PgRow) -> Self;
    fn to_row(&self) -> prettytable::Row;
    fn headers() -> prettytable::Row;
}

pub enum Query {
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
            Self::CacheHit => include_str!("../queries/cache_hit.sql"),
            Self::DbSettings => include_str!("../queries/db_settings.sql"),
            Self::BuffercacheStats => include_str!("../queries/buffercache_stats.sql"),
            Self::BuffercacheUsage => include_str!("../queries/buffercache_usage.sql"),
            Self::SslUsed => include_str!("../queries/ssl_used.sql"),
            Self::Connections => include_str!("../queries/connections.sql"),
            Self::Bloat => include_str!("../queries/bloat.sql"),
            Self::Blocking => include_str!("../queries/blocking.sql"),
            Self::Calls => include_str!("../queries/calls.sql"),
            Self::Extensions => include_str!("../queries/extensions.sql"),
            Self::TableCacheHit => include_str!("../queries/table_cache_hit.sql"),
            Self::Tables => include_str!("../queries/tables.sql"),
            Self::IndexCacheHit => include_str!("../queries/index_cache_hit.sql"),
            Self::Indexes => include_str!("../queries/indexes.sql"),
            Self::IndexSize => include_str!("../queries/index_size.sql"),
            Self::IndexUsage => include_str!("../queries/index_usage.sql"),
            Self::IndexScans => include_str!("../queries/index_scans.sql"),
            Self::NullIndexes => include_str!("../queries/null_indexes.sql"),
            Self::Locks => include_str!("../queries/locks.sql"),
            Self::AllLocks => include_str!("../queries/all_locks.sql"),
            Self::LongRunningQueries => include_str!("../queries/long_running_queries.sql"),
            Self::Mandelbrot => include_str!("../queries/mandelbrot.sql"),
            Self::Outliers => include_str!("../queries/outliers.sql"),
            Self::RecordsRank => include_str!("../queries/records_rank.sql"),
            Self::SeqScans => include_str!("../queries/seq_scans.sql"),
            Self::TableIndexScans => include_str!("../queries/table_index_scans.sql"),
            Self::TableIndexesSize => include_str!("../queries/table_indexes_size.sql"),
            Self::TableSize => include_str!("../queries/table_size.sql"),
            Self::TotalIndexSize => include_str!("../queries/total_index_size.sql"),
            Self::TotalTableSize => include_str!("../queries/total_table_size.sql"),
            Self::UnusedIndexes => include_str!("../queries/unused_indexes.sql"),
            Self::DuplicateIndexes => include_str!("../queries/duplicate_indexes.sql"),
            Self::VacuumStats => include_str!("../queries/vacuum_stats.sql"),
        }
    }
}

pub fn get_default_interval() -> PgInterval {
    PgInterval {
        microseconds: 0,
        days: 0,
        months: 0,
    }
}

pub fn get_default_schema() -> String {
    env::var("PG_EXTRAS_SCHEMA").unwrap_or("public".to_string())
}
