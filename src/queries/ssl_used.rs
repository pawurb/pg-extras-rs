use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone)]
pub struct SslUsed {
    pub ssl_used: bool,
}

impl Query for SslUsed {
    fn new(row: &PgRow) -> Self {
        Self {
            ssl_used: row.try_get("ssl_used").unwrap_or(false),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.ssl_used.to_string()]
    }

    fn headers() -> prettytable::Row {
        row!["ssl_used"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/ssl_used.sql").to_string()
    }
}
