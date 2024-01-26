use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct TableCacheHit {
    name: String,
    buffer_hits: i64,
    block_reads: i64,
    total_read: i64,
    ratio: String,
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

    fn read_file() -> &'static str {
        include_str!("../sql/table_cache_hit.sql")
    }
}
