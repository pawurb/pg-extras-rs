use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct TotalTableSize {
    name: String,
    size: String,
}

impl Tabular for TotalTableSize {
    fn new(row: &Row) -> Self {
        Self {
            name: row.get::<_, Option<String>>(0).unwrap_or_default(),
            size: row.get::<_, Option<String>>(1).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.size]
    }

    fn headers() -> prettytable::Row {
        row!["name", "size"]
    }
}
