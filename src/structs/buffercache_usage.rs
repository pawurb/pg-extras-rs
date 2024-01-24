use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct BuffercacheUsage {
    relname: String,
    buffers: i64,
}

impl Tabular for BuffercacheUsage {
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
}
