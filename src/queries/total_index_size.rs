use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone, serde::Serialize)]
pub struct TotalIndexSize {
    pub size: String,
}

impl Query for TotalIndexSize {
    fn new(row: &PgRow) -> Self {
        Self {
            size: row.try_get("size").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.size]
    }

    fn headers() -> prettytable::Row {
        row!["size"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/total_index_size.sql").to_string()
    }
}
