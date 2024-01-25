use crate::structs::shared::{get_default_interval, Query, Tabular};
use sqlx::postgres::{types::PgInterval, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Calls {
    qry: String,
    exec_time: PgInterval,
    prop_exec_time: String,
    ncalls: String,
    sync_io_time: PgInterval,
}

impl Tabular for Calls {
    const FILE_NAME: Query = Query::Calls;

    fn new(row: &PgRow) -> Self {
        Self {
            qry: row.try_get("qry").unwrap_or_default(),
            exec_time: row.try_get("exec_time").unwrap_or(get_default_interval()),
            prop_exec_time: row.try_get("prop_exec_time").unwrap_or_default(),
            ncalls: row.try_get("ncalls").unwrap_or_default(),
            sync_io_time: row
                .try_get("sync_io_time")
                .unwrap_or(get_default_interval()),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.qry,
            format!("{:?}", self.exec_time),
            self.prop_exec_time,
            self.ncalls,
            format!("{:?}", self.sync_io_time),
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
