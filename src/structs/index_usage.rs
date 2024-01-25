use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct IndexUsage {
    relname: String,
    percent_of_times_index_used: String,
    rows_in_table: i64,
}

impl Tabular for IndexUsage {
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

    fn read_file() -> &'static str {
        include_str!("../queries/index_usage.sql")
    }
}
