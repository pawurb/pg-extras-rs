use crate::structs::shared::Tabular;
use postgres::Row;

pub struct SeqScans {
    name: String,
    count: i64,
}

impl Tabular for SeqScans {
    const FILE_NAME: &'static str = "seq_scans";

    fn new(row: &Row) -> Self {
        SeqScans {
            name: row.get::<_, Option<String>>(0).unwrap_or_default(),
            count: row.get::<_, Option<i64>>(1).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.count]
    }

    fn headers() -> prettytable::Row {
        row!["name", "count"]
    }
}
