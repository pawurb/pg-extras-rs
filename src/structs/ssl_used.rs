use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct SslUsed {
    ssl_used: bool,
}

impl Tabular for SslUsed {
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
}
