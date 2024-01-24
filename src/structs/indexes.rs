use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Indexes {
    schemaname: String,
    indexname: String,
    tablename: String,
    columns: String,
}

impl Tabular for Indexes {
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
}
