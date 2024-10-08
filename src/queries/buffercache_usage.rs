use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone)]
pub struct BuffercacheUsage {
    pub relname: String,
    pub buffers: i64,
}

impl Query for BuffercacheUsage {
    fn new(row: &PgRow) -> Self {
        Self {
            relname: row.try_get("relname").unwrap_or_default(),
            buffers: row.try_get("buffers").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.relname, self.buffers]
    }

    fn headers() -> prettytable::Row {
        row!["relname", "buffers"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/buffercache_usage.sql").to_string()
    }
}
