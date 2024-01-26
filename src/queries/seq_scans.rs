use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct SeqScans {
    pub name: String,
    pub count: i64,
}

impl Query for SeqScans {
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

    fn read_file() -> String {
        include_str!("../sql/seq_scans.sql").to_string()
    }
}
