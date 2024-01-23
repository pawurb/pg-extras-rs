use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct SslUsed {
    ssl_used: bool,
}

impl Tabular for SslUsed {
    fn new(row: &Row) -> Self {
        Self {
            ssl_used: row.get::<_, Option<bool>>(0).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.ssl_used.to_string()]
    }

    fn headers() -> prettytable::Row {
        row!["ssl_used"]
    }
}
