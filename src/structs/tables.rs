use crate::structs::shared::{Query, Tabular};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Tables {
    tablename: String,
    schemaname: String,
}

impl Tabular for Tables {
    const FILE_NAME: Query = Query::Tables;

    fn new(row: &PgRow) -> Self {
        Self {
            tablename: row.try_get("tablename").unwrap_or_default(),
            schemaname: row.try_get("schemaname").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.tablename, self.schemaname]
    }

    fn headers() -> prettytable::Row {
        row!["tablename", "schemaname"]
    }
}
