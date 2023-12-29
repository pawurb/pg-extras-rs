use crate::structs::shared::Tabular;
use postgres::Row;

pub struct DuplicateIndexes {
    size: String,
    idx1: String,
    idx2: String,
    idx3: String,
    idx4: String,
}

impl Tabular for DuplicateIndexes {
    const FILE_NAME: &'static str = "duplicate_indexes";

    fn new(row: &Row) -> Self {
        DuplicateIndexes {
            size: row.get::<_, Option<String>>(0).unwrap_or_default(),
            idx1: row.get::<_, Option<String>>(1).unwrap_or_default(),
            idx2: row.get::<_, Option<String>>(2).unwrap_or_default(),
            idx3: row.get::<_, Option<String>>(3).unwrap_or_default(),
            idx4: row.get::<_, Option<String>>(4).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.size, self.idx1, self.idx2, self.idx3, self.idx4]
    }

    fn headers() -> prettytable::Row {
        row!["size", "idx1", "idx2", "idx3", "idx4"]
    }
}
