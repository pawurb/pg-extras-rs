use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::types::BigDecimal;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Bloat {
    typefield: String,
    schemaname: String,
    object_name: String,
    bloat: BigDecimal,
    waste: String,
}

impl Tabular for Bloat {
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

    fn read_file() -> &'static str {
        include_str!("../queries/bloat.sql")
    }
}
