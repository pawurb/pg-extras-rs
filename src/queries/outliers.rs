use crate::queries::shared::{get_default_interval, Query};
use sqlx::postgres::{types::PgInterval, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Outliers {
    total_exec_time: PgInterval,
    prop_exec_time: String,
    ncalls: String,
    sync_io_time: PgInterval,
    query: String,
}

impl Query for Outliers {
    fn new(row: &PgRow) -> Self {
        Self {
            total_exec_time: row
                .try_get("total_exec_time")
                .unwrap_or(get_default_interval()),
            prop_exec_time: row.try_get("prop_exec_time").unwrap_or_default(),
            ncalls: row.try_get("ncalls").unwrap_or_default(),
            sync_io_time: row
                .try_get("sync_io_time")
                .unwrap_or(get_default_interval()),
            query: row.try_get("query").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            format!("{:?}", self.total_exec_time),
            self.prop_exec_time,
            self.ncalls,
            format!("{:?}", self.sync_io_time),
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

    fn read_file() -> &'static str {
        include_str!("../sql/outliers.sql")
    }
}
