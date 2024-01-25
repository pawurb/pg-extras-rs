use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct TableIndexesSize {
    table: String,
    index_size: String,
}

impl Tabular for TableIndexesSize {
    fn new(row: &PgRow) -> Self {
        Self {
            table: row.try_get("table").unwrap_or_default(),
            index_size: row.try_get("index_size").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.table, self.index_size]
    }

    fn headers() -> prettytable::Row {
        row!["table", "index_size"]
    }

    fn read_file() -> &'static str {
        include_str!("../queries/table_indexes_size.sql")
    }
}
