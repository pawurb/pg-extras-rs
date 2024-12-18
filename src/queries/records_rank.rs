use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone, serde::Serialize)]
pub struct RecordsRank {
    pub name: String,
    pub esiimated_count: i64,
}

impl Query for RecordsRank {
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

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/records_rank.sql").to_string()
    }
}
