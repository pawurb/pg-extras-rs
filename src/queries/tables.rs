use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Tables {
    pub tablename: String,
    pub schemaname: String,
}

impl Query for Tables {
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

    fn read_file() -> String {
        include_str!("../sql/tables.sql").to_string()
    }
}
