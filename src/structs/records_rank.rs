use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct RecordsRank {
    name: String,
    esiimated_count: i64,
}

impl Tabular for RecordsRank {
    fn new(row: &Row) -> Self {
        Self {
            name: row.get::<_, Option<String>>(0).unwrap_or_default(),
            esiimated_count: row.get::<_, Option<i64>>(1).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.esiimated_count]
    }

    fn headers() -> prettytable::Row {
        row!["name", "estimated_count"]
    }
}
