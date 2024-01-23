use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct TotalIndexSize {
    size: String,
}

impl Tabular for TotalIndexSize {
    fn new(row: &Row) -> Self {
        Self {
            size: row.get::<_, Option<String>>(0).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.size]
    }

    fn headers() -> prettytable::Row {
        row!["size"]
    }
}
