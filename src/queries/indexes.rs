use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone)]
pub struct Indexes {
    pub schemaname: String,
    pub indexname: String,
    pub tablename: String,
    pub columns: String,
}

impl Query for Indexes {
    fn new(row: &PgRow) -> Self {
        Self {
            schemaname: row.try_get("schemaname").unwrap_or_default(),
            indexname: row.try_get("indexname").unwrap_or_default(),
            tablename: row.try_get("tablename").unwrap_or_default(),
            columns: row.try_get("columns").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.schemaname,
            self.indexname,
            self.tablename,
            self.columns
        ]
    }

    fn headers() -> prettytable::Row {
        row!["schemaname", "indexname", "tablename", "columns"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/indexes.sql").to_string()
    }
}
