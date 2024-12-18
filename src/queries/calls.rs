use crate::queries::shared::{get_default_interval, Query};
use crate::PgStatsVersion;
use sqlx::postgres::{types::PgInterval, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Calls {
    pub qry: String,
    pub exec_time: PgInterval,
    pub prop_exec_time: String,
    pub ncalls: String,
    pub sync_io_time: PgInterval,
}

impl serde::Serialize for Calls {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Calls", 5)?;
        state.serialize_field("qry", &self.qry)?;
        state.serialize_field("exec_time", &format!("{:?}", self.exec_time))?;
        state.serialize_field("prop_exec_time", &self.prop_exec_time)?;
        state.serialize_field("ncalls", &self.ncalls)?;
        state.serialize_field("sync_io_time", &format!("{:?}", self.sync_io_time))?;
        state.end()
    }
}

impl Query for Calls {
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

    fn read_file(pg_statement_version: Option<PgStatsVersion>) -> String {
        let default = include_str!("../sql/calls.sql");

        match pg_statement_version {
            Some(PgStatsVersion::Legacy) => include_str!("../sql/calls_legacy.sql"),
            Some(PgStatsVersion::Standard) => default,
            Some(PgStatsVersion::Pg17) => include_str!("../sql/calls_17.sql"),
            None => default,
        }
        .to_string()
    }
}
