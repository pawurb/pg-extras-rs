use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone, serde::Serialize)]
pub struct TableCacheHit {
    pub name: String,
    pub buffer_hits: i64,
    pub block_reads: i64,
    pub total_read: i64,
    pub ratio: String,
}

impl Query for TableCacheHit {
    fn new(row: &PgRow) -> Self {
        Self {
            name: row.try_get("name").unwrap_or_default(),
            buffer_hits: row.try_get("buffer_hits").unwrap_or_default(),
            block_reads: row.try_get("block_reads").unwrap_or_default(),
            total_read: row.try_get("total_read").unwrap_or_default(),
            ratio: row.try_get("ratio").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.name,
            self.buffer_hits,
            self.block_reads,
            self.total_read,
            self.ratio
        ]
    }

    fn headers() -> prettytable::Row {
        row!["name", "buffer_hits", "block_reads", "total_read", "ratio"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/table_cache_hit.sql").to_string()
    }
}
