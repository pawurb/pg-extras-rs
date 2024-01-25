use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct IndexScans {
    schemaname: String,
    table: String,
    index: String,
    index_size: String,
    index_scans: i64,
}

impl Tabular for IndexScans {
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

    fn read_file() -> &'static str {
        include_str!("../queries/index_scans.sql")
    }
}
