use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone)]
pub struct DuplicateIndexes {
    pub size: String,
    pub idx1: String,
    pub idx2: String,
    pub idx3: String,
    pub idx4: String,
}

impl Query for DuplicateIndexes {
    fn new(row: &PgRow) -> Self {
        Self {
            size: row.try_get("size").unwrap_or_default(),
            idx1: row.try_get("idx1").unwrap_or_default(),
            idx2: row.try_get("idx2").unwrap_or_default(),
            idx3: row.try_get("idx3").unwrap_or_default(),
            idx4: row.try_get("idx4").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.size, self.idx1, self.idx2, self.idx3, self.idx4]
    }

    fn headers() -> prettytable::Row {
        row!["size", "idx1", "idx2", "idx3", "idx4"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/duplicate_indexes.sql").to_string()
    }
}
