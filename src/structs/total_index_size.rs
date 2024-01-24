use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct TotalIndexSize {
    size: String,
}

impl Tabular for TotalIndexSize {
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
}
