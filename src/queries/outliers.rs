use crate::queries::shared::{get_default_interval, Query};
use crate::PgStatsVersion;
use sqlx::postgres::{types::PgInterval, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Outliers {
    pub total_exec_time: PgInterval,
    pub prop_exec_time: String,
    pub ncalls: String,
    pub sync_io_time: PgInterval,
    pub query: String,
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

    fn read_file(pg_statement_version: Option<PgStatsVersion>) -> String {
        let default = include_str!("../sql/outliers.sql");

        match pg_statement_version {
            Some(PgStatsVersion::Legacy) => include_str!("../sql/outliers_legacy.sql"),
            Some(PgStatsVersion::Standard) => default,
            Some(PgStatsVersion::Pg17) => include_str!("../sql/outliers_17.sql"),
            None => default,
        }
        .to_string()
    }
}
