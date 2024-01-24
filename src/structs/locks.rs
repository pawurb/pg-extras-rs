use crate::structs::shared::{get_default_interval, Tabular};
use sqlx::postgres::{types::PgInterval, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Locks {
    pid: i32,
    relname: String,
    transactionid: String,
    granted: bool,
    mode: String,
    query_snippet: String,
    age: PgInterval,
    application: String,
}

impl Tabular for Locks {
    fn new(row: &PgRow) -> Self {
        Self {
            pid: row.try_get("pid").unwrap_or_default(),
            relname: row.try_get("relname").unwrap_or_default(),
            transactionid: row.try_get("transactionid").unwrap_or_default(),
            granted: row.try_get("granted").unwrap_or_default(),
            mode: row.try_get("mode").unwrap_or_default(),
            query_snippet: row.try_get("query_snippet").unwrap_or_default(),
            age: row.try_get("age").unwrap_or(get_default_interval()),
            application: row.try_get("application").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.pid,
            self.relname,
            self.transactionid,
            self.granted,
            self.mode,
            self.query_snippet,
            format!("{:?}", self.age),
            self.application
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "pid",
            "relname",
            "transactionid",
            "granted",
            "mode",
            "query_snippet",
            "age",
            "application"
        ]
    }
}
