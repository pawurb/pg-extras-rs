use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug)]
pub struct RecordsRank {
    name: String,
    esiimated_count: i64,
}

impl Tabular for RecordsRank {
    const FILE_NAME: &'static str = "records_rank";

    fn new(row: &Row) -> Self {
        RecordsRank {
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
