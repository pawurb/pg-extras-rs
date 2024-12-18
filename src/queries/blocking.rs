use crate::queries::shared::{get_default_interval, Query};
use crate::PgStatsVersion;
use sqlx::postgres::{types::PgInterval, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Blocking {
    pub blocked_pid: i32,
    pub blocking_statement: String,
    pub blocking_duration: PgInterval,
    pub blocking_pid: i32,
    pub blocked_statement: String,
    pub blocked_duration: PgInterval,
    pub blocked_sql_app: String,
    pub blocking_sql_app: String,
}

impl serde::Serialize for Blocking {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Blocking", 8)?;
        state.serialize_field("blocked_pid", &self.blocked_pid)?;
        state.serialize_field("blocking_statement", &self.blocking_statement)?;
        state.serialize_field(
            "blocking_duration",
            &format!("{:?}", self.blocking_duration),
        )?;
        state.serialize_field("blocking_pid", &self.blocking_pid)?;
        state.serialize_field("blocked_statement", &self.blocked_statement)?;
        state.serialize_field("blocked_duration", &format!("{:?}", self.blocked_duration))?;
        state.serialize_field("blocked_sql_app", &self.blocked_sql_app)?;
        state.serialize_field("blocking_sql_app", &self.blocking_sql_app)?;
        state.end()
    }
}

impl Query for Blocking {
    fn new(row: &PgRow) -> Self {
        Self {
            blocked_pid: row.try_get("blocked_pid").unwrap_or_default(),
            blocking_statement: row.try_get("blocking_statement").unwrap_or_default(),
            blocking_duration: row
                .try_get("blocking_duration")
                .unwrap_or(get_default_interval()),
            blocking_pid: row.try_get("blocking_pid").unwrap_or_default(),
            blocked_statement: row.try_get("blocked_statement").unwrap_or_default(),
            blocked_duration: row
                .try_get("blocked_duration")
                .unwrap_or(get_default_interval()),
            blocked_sql_app: row.try_get("blocked_sql_app").unwrap_or_default(),
            blocking_sql_app: row.try_get("blocking_sql_app").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.blocked_pid,
            self.blocking_statement,
            format!("{:?}", self.blocking_duration),
            self.blocking_pid,
            self.blocked_statement,
            format!("{:?}", self.blocked_duration),
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

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/blocking.sql").to_string()
    }
}
