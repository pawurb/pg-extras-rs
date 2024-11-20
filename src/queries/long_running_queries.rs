use crate::queries::shared::{get_default_interval, Query};
use crate::PgStatsVersion;
use sqlx::postgres::{types::PgInterval, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct LongRunningQueries {
    pub pid: i32,
    pub duration: PgInterval,
    pub query: String,
}

impl serde::Serialize for LongRunningQueries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("LongRunningQueries", 3)?;
        state.serialize_field("pid", &self.pid)?;
        state.serialize_field("duration", &format!("{:?}", self.duration))?;
        state.serialize_field("query", &self.query)?;
        state.end()
    }
}

impl Query for LongRunningQueries {
    fn new(row: &PgRow) -> Self {
        Self {
            pid: row.try_get("pid").unwrap_or_default(),
            duration: row.try_get("duration").unwrap_or(get_default_interval()),
            query: row.try_get("query").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.pid, format!("{:?}", self.duration), self.query]
    }

    fn headers() -> prettytable::Row {
        row!["pid", "duration", "query"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/long_running_queries.sql").to_string()
    }
}
