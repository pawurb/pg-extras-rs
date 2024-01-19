use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct IndexSize {
    name: String,
    size: String,
    schema: String,
}

impl Tabular for IndexSize {
    const FILE_NAME: &'static str = "index_size";

    fn new(row: &Row) -> Self {
        Self {
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
