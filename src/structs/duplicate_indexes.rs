use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct DuplicateIndexes {
    size: String,
    idx1: String,
    idx2: String,
    idx3: String,
    idx4: String,
}

impl Tabular for DuplicateIndexes {
    fn new(row: &PgRow) -> Self {
        Self {
            size: row.try_get("size").unwrap_or_default(),
            idx1: row.try_get("idx1").unwrap_or_default(),
            idx2: row.try_get("idx2").unwrap_or_default(),
            idx3: row.try_get("idx3").unwrap_or_default(),
            idx4: row.try_get("idx4").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.size, self.idx1, self.idx2, self.idx3, self.idx4]
    }

    fn headers() -> prettytable::Row {
        row!["size", "idx1", "idx2", "idx3", "idx4"]
    }
}
