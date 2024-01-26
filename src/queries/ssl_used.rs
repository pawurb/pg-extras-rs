use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct SslUsed {
    ssl_used: bool,
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

    fn read_file() -> &'static str {
        include_str!("../sql/ssl_used.sql")
    }
}
