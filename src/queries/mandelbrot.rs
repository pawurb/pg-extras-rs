use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone)]
pub struct Mandelbrot {
    pub array_to_string: String,
}

impl Query for Mandelbrot {
    fn new(row: &PgRow) -> Self {
        Self {
            array_to_string: row.try_get("array_to_string").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.array_to_string]
    }

    fn headers() -> prettytable::Row {
        row!["array_to_string"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/mandelbrot.sql").to_string()
    }
}
