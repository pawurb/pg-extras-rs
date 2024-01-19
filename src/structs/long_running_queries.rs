use crate::structs::shared::{get_default_interval, Tabular};
use pg_interval::Interval;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct LongRunningQueries {
    pid: String,
    duration: String,
    query: String,
}

impl Tabular for LongRunningQueries {
    const FILE_NAME: &'static str = "long_running_queries";

    fn new(row: &Row) -> Self {
        Self {
            pid: row.get::<_, Option<i32>>(0).unwrap_or_default().to_string(),
            duration: row
                .get::<_, Option<Interval>>(1)
                .unwrap_or(get_default_interval())
                .to_iso_8601(),
            query: row.get::<_, Option<String>>(2).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.pid, self.duration, self.query]
    }

    fn headers() -> prettytable::Row {
        row!["pid", "duration", "query"]
    }
}
