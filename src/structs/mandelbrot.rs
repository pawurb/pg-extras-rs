use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct Mandelbrot {
    array_to_string: String,
}

impl Tabular for Mandelbrot {
    fn new(row: &Row) -> Self {
        Self {
            array_to_string: row.get::<_, Option<String>>(0).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.array_to_string]
    }

    fn headers() -> prettytable::Row {
        row!["array_to_string"]
    }
}
