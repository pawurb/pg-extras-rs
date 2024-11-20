use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::postgres::PgRow;
use sqlx::types::BigDecimal;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Bloat {
    pub typefield: String,
    pub schemaname: String,
    pub object_name: String,
    pub bloat: BigDecimal,
    pub waste: String,
}

impl serde::Serialize for Bloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Bloat", 5)?;
        state.serialize_field("typefield", &self.typefield)?;
        state.serialize_field("schemaname", &self.schemaname)?;
        state.serialize_field("object_name", &self.object_name)?;
        state.serialize_field("bloat", &format!("{}", self.bloat))?;
        state.serialize_field("waste", &self.waste)?;
        state.end()
    }
}

impl Query for Bloat {
    fn new(row: &PgRow) -> Self {
        Self {
            typefield: row.try_get("type").unwrap_or_default(),
            schemaname: row.try_get("schemaname").unwrap_or_default(),
            object_name: row.try_get("object_name").unwrap_or_default(),
            bloat: row.try_get("bloat").unwrap_or_default(),
            waste: row.try_get("waste").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.typefield,
            self.schemaname,
            self.object_name,
            self.bloat,
            self.waste
        ]
    }

    fn headers() -> prettytable::Row {
        row!["type", "schemaname", "object_name", "bloat", "waste"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/bloat.sql").to_string()
    }
}
