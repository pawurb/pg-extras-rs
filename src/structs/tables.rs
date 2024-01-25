use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Tables {
    tablename: String,
    schemaname: String,
}

impl Tabular for Tables {
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

    fn read_file() -> &'static str {
        include_str!("../queries/tables.sql")
    }
}
