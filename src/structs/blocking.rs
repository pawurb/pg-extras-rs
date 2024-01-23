use crate::structs::shared::{get_default_interval, Tabular};
use pg_interval::Interval;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct Blocking {
    blocked_pid: i32,
    blocking_statement: String,
    blocking_duration: Interval,
    blocking_pid: i32,
    blocked_statement: String,
    blocked_duration: Interval,
    blocked_sql_app: String,
    blocking_sql_app: String,
}

impl Tabular for Blocking {
    fn new(row: &Row) -> Self {
        Self {
            blocked_pid: row.get::<_, Option<i32>>(0).unwrap_or_default(),
            blocking_statement: row.get::<_, Option<String>>(1).unwrap_or_default(),
            blocking_duration: row
                .get::<_, Option<Interval>>(2)
                .unwrap_or(get_default_interval()),
            blocking_pid: row.get::<_, Option<i32>>(3).unwrap_or_default(),
            blocked_statement: row.get::<_, Option<String>>(4).unwrap_or_default(),
            blocked_duration: row
                .get::<_, Option<Interval>>(5)
                .unwrap_or(get_default_interval()),
            blocked_sql_app: row.get::<_, Option<String>>(6).unwrap_or_default(),
            blocking_sql_app: row.get::<_, Option<String>>(7).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.blocked_pid,
            self.blocking_statement,
            self.blocking_duration.to_iso_8601(),
            self.blocking_pid,
            self.blocked_statement,
            self.blocked_duration.to_iso_8601(),
            self.blocked_sql_app,
            self.blocking_sql_app
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "blocked_pid",
            "blocking_statement",
            "blocking_duration",
            "blocking_pid",
            "blocked_statement",
            "blocked_duration",
            "blocked_sql_app",
            "blocking_sql_app"
        ]
    }
}
