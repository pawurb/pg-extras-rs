use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::postgres::PgRow;
use sqlx::types::BigDecimal;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct CacheHit {
    pub name: String,
    pub ratio: BigDecimal,
}

impl serde::Serialize for CacheHit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("CacheHit", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("ratio", &format!("{:?}", self.ratio))?;
        state.end()
    }
}

impl Query for CacheHit {
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

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/cache_hit.sql").to_string()
    }
}
