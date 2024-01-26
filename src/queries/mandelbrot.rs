use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Mandelbrot {
    array_to_string: String,
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

    fn read_file() -> &'static str {
        include_str!("../sql/mandelbrot.sql")
    }
}
