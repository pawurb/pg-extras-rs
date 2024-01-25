use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::types::BigDecimal;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct CacheHit {
    name: String,
    ratio: BigDecimal,
}

impl Tabular for CacheHit {
    fn new(row: &PgRow) -> Self {
        Self {
            name: row.try_get("name").unwrap_or_default(),
            ratio: row.try_get("ratio").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.ratio]
    }

    fn headers() -> prettytable::Row {
        row!["name", "ratio"]
    }

    fn read_file() -> &'static str {
        include_str!("../queries/cache_hit.sql")
    }
}
