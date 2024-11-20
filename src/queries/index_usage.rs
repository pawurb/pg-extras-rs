use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone, serde::Serialize)]
pub struct IndexUsage {
    pub relname: String,
    pub percent_of_times_index_used: String,
    pub rows_in_table: i64,
}

impl Query for IndexUsage {
    fn new(row: &PgRow) -> Self {
        Self {
            relname: row.try_get("relname").unwrap_or_default(),
            percent_of_times_index_used: row
                .try_get("percent_of_times_index_used")
                .unwrap_or_default(),
            rows_in_table: row.try_get("rows_in_table").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.relname,
            self.percent_of_times_index_used,
            self.rows_in_table.to_string()
        ]
    }

    fn headers() -> prettytable::Row {
        row!["relname", "percent_of_times_index_used", "rows_in_table"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/index_usage.sql").to_string()
    }
}
