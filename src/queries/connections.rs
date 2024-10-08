use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone)]
pub struct Connections {
    pub username: String,
    pub pid: i32,
    pub client_addr: String,
}

impl Query for Connections {
    fn new(row: &PgRow) -> Self {
        Self {
            username: row.try_get("username").unwrap_or_default(),
            pid: row.try_get("pid").unwrap_or_default(),
            client_addr: row.try_get("client_addr").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.username, self.pid, self.client_addr]
    }

    fn headers() -> prettytable::Row {
        row!["username", "pid", "client_addr"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/connections.sql").to_string()
    }
}
