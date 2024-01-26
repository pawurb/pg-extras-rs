use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct TableSize {
    name: String,
    size: String,
    schema: String,
}

impl Query for TableSize {
    fn new(row: &PgRow) -> Self {
        Self {
            name: row.try_get("name").unwrap_or_default(),
            size: row.try_get("size").unwrap_or_default(),
            schema: row.try_get("schema").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.size, self.schema]
    }

    fn headers() -> prettytable::Row {
        row!["name", "size", "schema"]
    }

    fn read_file() -> &'static str {
        include_str!("../sql/table_size.sql")
    }
}
