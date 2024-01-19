use crate::structs::shared::{get_default_interval, Tabular};
use pg_interval::Interval;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct Calls {
    qry: String,
    exec_time: Interval,
    prop_exec_time: String,
    ncalls: String,
    sync_io_time: Interval,
}

impl Tabular for Calls {
    const FILE_NAME: &'static str = "calls";

    fn new(row: &Row) -> Self {
        Self {
            qry: row.get::<_, Option<String>>(0).unwrap_or_default(),
            exec_time: row
                .get::<_, Option<Interval>>(1)
                .unwrap_or(get_default_interval()),
            prop_exec_time: row.get::<_, Option<String>>(2).unwrap_or_default(),
            ncalls: row.get::<_, Option<String>>(3).unwrap_or_default(),
            sync_io_time: row
                .get::<_, Option<Interval>>(4)
                .unwrap_or(get_default_interval()),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.qry,
            self.exec_time.to_iso_8601(),
            self.prop_exec_time,
            self.ncalls,
            self.sync_io_time.to_iso_8601()
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "qry",
            "exec_time",
            "prop_exec_time",
            "ncalls",
            "sync_io_time"
        ]
    }
}
