use crate::Tabular;
use postgres::Row;

pub struct TableSize {
    name: String,
    size: String,
    schema: String,
}

impl Tabular for TableSize {
    const FILE_NAME: &'static str = "table_size";

    fn new(row: &Row) -> Self {
        TableSize {
            name: row.get::<_, Option<String>>(0).unwrap_or_default(),
            size: row.get::<_, Option<String>>(1).unwrap_or_default(),
            schema: row.get::<_, Option<String>>(2).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.size, self.schema]
    }

    fn headers() -> prettytable::Row {
        row!["name", "size", "schema"]
    }
}
