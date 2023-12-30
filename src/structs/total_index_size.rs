use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug)]
pub struct TotalIndexSize {
    size: String,
}

impl Tabular for TotalIndexSize {
    const FILE_NAME: &'static str = "total_index_size";

    fn new(row: &Row) -> Self {
        TotalIndexSize {
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
