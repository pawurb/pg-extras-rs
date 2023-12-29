use crate::structs::shared::Tabular;
use postgres::Row;

pub struct TotalTableSize {
    name: String,
    size: String,
}

impl Tabular for TotalTableSize {
    const FILE_NAME: &'static str = "total_table_size";

    fn new(row: &Row) -> Self {
        TotalTableSize {
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
