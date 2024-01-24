use crate::structs::shared::{get_default_interval, Tabular};
use sqlx::postgres::types::PgInterval;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct LongRunningQueries {
    pid: i32,
    duration: PgInterval,
    query: String,
}

impl Tabular for LongRunningQueries {
    fn new(row: &PgRow) -> Self {
        Self {
            pid: row.try_get("pid").unwrap_or_default(),
            duration: row.try_get("duration").unwrap_or(get_default_interval()),
            query: row.try_get("query").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.pid, format!("{:?}", self.duration), self.query]
    }

    fn headers() -> prettytable::Row {
        row!["pid", "duration", "query"]
    }
}
