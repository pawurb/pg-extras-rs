use crate::queries::shared::{get_default_interval, Query};
use sqlx::postgres::{types::PgInterval, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct LongRunningQueries {
    pid: i32,
    duration: PgInterval,
    query: String,
}

impl Query for LongRunningQueries {
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

    fn read_file() -> &'static str {
        include_str!("../sql/long_running_queries.sql")
    }
}
