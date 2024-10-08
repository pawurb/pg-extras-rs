use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone)]
pub struct IndexScans {
    pub schemaname: String,
    pub table: String,
    pub index: String,
    pub index_size: String,
    pub index_scans: i64,
}

impl Query for IndexScans {
    fn new(row: &PgRow) -> Self {
        Self {
            schemaname: row.try_get("schemaname").unwrap_or_default(),
            table: row.try_get("table").unwrap_or_default(),
            index: row.try_get("index").unwrap_or_default(),
            index_size: row.try_get("index_size").unwrap_or_default(),
            index_scans: row.try_get("index_scans").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.schemaname,
            self.table,
            self.index,
            self.index_size,
            self.index_scans.to_string()
        ]
    }

    fn headers() -> prettytable::Row {
        row!["schemaname", "table", "index", "index_size", "index_scans"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/index_scans.sql").to_string()
    }
}
