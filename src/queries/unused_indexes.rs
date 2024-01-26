use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct UnusedIndexes {
    pub table: String,
    pub index: String,
    pub index_size: String,
    pub index_scans: i64,
}

impl Query for UnusedIndexes {
    fn new(row: &PgRow) -> Self {
        Self {
            table: row.try_get("table").unwrap_or_default(),
            index: row.try_get("index").unwrap_or_default(),
            index_size: row.try_get("index_size").unwrap_or_default(),
            index_scans: row.try_get("index_scans").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.table, self.index, self.index_size, self.index_scans]
    }

    fn headers() -> prettytable::Row {
        row!["table", "index", "index_size", "index_scans"]
    }

    fn read_file() -> String {
        include_str!("../sql/unused_indexes.sql").to_string()
    }
}
