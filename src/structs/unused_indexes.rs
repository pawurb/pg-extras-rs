use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct UnusedIndexes {
    table: String,
    index: String,
    index_size: String,
    index_scans: i64,
}

impl Tabular for UnusedIndexes {
    const FILE_NAME: &'static str = "unused_indexes";

    fn new(row: &Row) -> Self {
        UnusedIndexes {
            table: row.get::<_, Option<String>>(0).unwrap_or_default(),
            index: row.get::<_, Option<String>>(1).unwrap_or_default(),
            index_size: row.get::<_, Option<String>>(2).unwrap_or_default(),
            index_scans: row.get::<_, Option<i64>>(3).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.table, self.index, self.index_size, self.index_scans]
    }

    fn headers() -> prettytable::Row {
        row!["table", "index", "index_size", "index_scans"]
    }
}
