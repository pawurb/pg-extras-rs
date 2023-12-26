use crate::Tabular;
use postgres::Row;

pub struct Mandelbrot {
    array_to_string: String,
}

impl Tabular for Mandelbrot {
    const FILE_NAME: &'static str = "mandelbrot";

    fn new(row: &Row) -> Self {
        Mandelbrot {
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
