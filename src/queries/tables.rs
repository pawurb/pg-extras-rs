use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone, serde::Serialize)]
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

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/tables.sql").to_string()
    }
}
