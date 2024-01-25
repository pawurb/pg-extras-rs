use crate::structs::shared::{Query, Tabular};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct TableIndexScans {
    name: String,
    count: i64,
}

impl Tabular for TableIndexScans {
    const FILE_NAME: Query = Query::TableIndexScans;

    fn new(row: &PgRow) -> Self {
        Self {
            name: row.try_get("name").unwrap_or_default(),
            count: row.try_get("count").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.count]
    }

    fn headers() -> prettytable::Row {
        row!["name", "count"]
    }
}
