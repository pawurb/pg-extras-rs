use crate::structs::shared::{get_default_interval, Tabular};
use pg_interval::Interval;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct AllLocks {
    pid: String,
    relname: String,
    transactionid: String,
    granted: String,
    mode: String,
    query_snippet: String,
    age: String,
    application: String,
}

impl Tabular for AllLocks {
    const FILE_NAME: &'static str = "all_locks";

    fn new(row: &Row) -> Self {
        Self {
            pid: row.get::<_, Option<i32>>(0).unwrap_or_default().to_string(),
            relname: row.get::<_, Option<String>>(1).unwrap_or_default(),
            transactionid: row.get::<_, Option<String>>(2).unwrap_or_default(),
            granted: row
                .get::<_, Option<bool>>(3)
                .unwrap_or_default()
                .to_string(),
            mode: row.get::<_, Option<String>>(4).unwrap_or_default(),
            query_snippet: row.get::<_, Option<String>>(5).unwrap_or_default(),
            age: row
                .get::<_, Option<Interval>>(6)
                .unwrap_or(get_default_interval())
                .to_iso_8601(),
            application: row.get::<_, Option<String>>(7).unwrap_or_default(),
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
            self.age,
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
