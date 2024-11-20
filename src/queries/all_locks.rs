use crate::queries::shared::{get_default_interval, Query};
use crate::PgStatsVersion;
use sqlx::postgres::{types::PgInterval, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct AllLocks {
    pub pid: i32,
    pub relname: String,
    pub transactionid: String,
    pub granted: bool,
    pub mode: String,
    pub query_snippet: String,
    pub age: PgInterval,
    pub application: String,
}

impl serde::Serialize for AllLocks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AllLocks", 8)?;
        state.serialize_field("pid", &self.pid)?;
        state.serialize_field("relname", &self.relname)?;
        state.serialize_field("transactionid", &self.transactionid)?;
        state.serialize_field("granted", &self.granted)?;
        state.serialize_field("mode", &self.mode)?;
        state.serialize_field("query_snippet", &self.query_snippet)?;
        state.serialize_field("age", &format!("{:?}", self.age))?;
        state.serialize_field("application", &self.application)?;
        state.end()
    }
}

impl Query for AllLocks {
    fn new(row: &PgRow) -> Self {
        Self {
            pid: row.try_get("pid").unwrap_or_default(),
            relname: row.try_get("relname").unwrap_or_default(),
            transactionid: row.try_get("transactionid").unwrap_or_default(),
            granted: row.try_get("granted").unwrap_or_default(),
            mode: row.try_get("mode").unwrap_or_default(),
            query_snippet: row.try_get("query_snippet").unwrap_or_default(),
            age: row.try_get("age").unwrap_or(get_default_interval()),
            application: row.try_get("application").unwrap_or_default(),
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
            format!("{:?}", self.age),
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

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/all_locks.sql").to_string()
    }
}
