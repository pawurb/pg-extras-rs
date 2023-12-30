use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug)]
pub struct Indexes {
    schemaname: String,
    indexname: String,
    tablename: String,
    columns: String,
}

impl Tabular for Indexes {
    const FILE_NAME: &'static str = "indexes";

    fn new(row: &Row) -> Self {
        Indexes {
            schemaname: row.get::<_, Option<String>>(0).unwrap_or_default(),
            indexname: row.get::<_, Option<String>>(1).unwrap_or_default(),
            tablename: row.get::<_, Option<String>>(2).unwrap_or_default(),
            columns: row.get::<_, Option<String>>(3).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.schemaname,
            self.indexname,
            self.tablename,
            self.columns
        ]
    }

    fn headers() -> prettytable::Row {
        row!["schemaname", "indexname", "tablename", "columns"]
    }
}
