use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct TotalTableSize {
    name: String,
    size: String,
}

impl Query for TotalTableSize {
    fn new(row: &PgRow) -> Self {
        Self {
            name: row.try_get("name").unwrap_or_default(),
            size: row.try_get("size").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.size]
    }

    fn headers() -> prettytable::Row {
        row!["name", "size"]
    }

    fn read_file() -> String {
        include_str!("../sql/total_table_size.sql").to_string()
    }
}
