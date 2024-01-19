use crate::structs::shared::{get_default_interval, Tabular};
use pg_interval::Interval;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct Outliers {
    total_exec_time: Interval,
    prop_exec_time: String,
    ncalls: String,
    sync_io_time: Interval,
    query: String,
}

impl Tabular for Outliers {
    const FILE_NAME: &'static str = "outliers";

    fn new(row: &Row) -> Self {
        Self {
            total_exec_time: row
                .get::<_, Option<Interval>>(0)
                .unwrap_or(get_default_interval()),
            prop_exec_time: row.get::<_, Option<String>>(1).unwrap_or_default(),
            ncalls: row.get::<_, Option<String>>(2).unwrap_or_default(),
            sync_io_time: row
                .get::<_, Option<Interval>>(3)
                .unwrap_or(get_default_interval()),
            query: row.get::<_, Option<String>>(4).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.total_exec_time.to_iso_8601(),
            self.prop_exec_time,
            self.ncalls,
            self.sync_io_time.to_iso_8601(),
            self.query
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "query",
            "exec_time",
            "prop_exec_time",
            "ncalls",
            "sync_io_time"
        ]
    }
}
