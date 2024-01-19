use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct Tables {
    tablename: String,
    schemaname: String,
}

impl Tabular for Tables {
    const FILE_NAME: &'static str = "tables";

    fn new(row: &Row) -> Self {
        Tables {
            tablename: row.get::<_, Option<String>>(0).unwrap_or_default(),
            schemaname: row.get::<_, Option<String>>(1).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.tablename, self.schemaname]
    }

    fn headers() -> prettytable::Row {
        row!["tablename", "schemaname"]
    }
}
