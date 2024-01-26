use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct BuffercacheStats {
    pub relname: String,
    pub buffered: String,
    pub buffer_percent: f64,
    pub percent_of_relation: f64,
}

impl Query for BuffercacheStats {
    fn new(row: &PgRow) -> Self {
        Self {
            relname: row.try_get("relname").unwrap_or_default(),
            buffered: row.try_get("buffered").unwrap_or_default(),
            buffer_percent: row.try_get("buffer_percent").unwrap_or_default(),
            percent_of_relation: row.try_get("percent_of_relation").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.relname,
            self.buffered,
            self.buffer_percent.to_string(),
            self.percent_of_relation.to_string()
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "relname",
            "buffered",
            "buffer_percent",
            "percent_of_relation"
        ]
    }

    fn read_file() -> String {
        include_str!("../sql/buffercache_stats.sql")
            .to_string()
            .to_string()
    }
}
