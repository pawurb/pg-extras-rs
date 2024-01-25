use crate::structs::shared::{Query, Tabular};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct RecordsRank {
    name: String,
    esiimated_count: i64,
}

impl Tabular for RecordsRank {
    const FILE_NAME: Query = Query::RecordsRank;

    fn new(row: &PgRow) -> Self {
        Self {
            name: row.try_get("name").unwrap_or_default(),
            esiimated_count: row.try_get("estimated_count").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.esiimated_count]
    }

    fn headers() -> prettytable::Row {
        row!["name", "estimated_count"]
    }
}
