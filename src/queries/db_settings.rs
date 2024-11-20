use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone, serde::Serialize)]
pub struct DbSettings {
    pub name: String,
    pub setting: String,
    pub unit: String,
    pub short_desc: String,
}

impl Query for DbSettings {
    fn new(row: &PgRow) -> Self {
        Self {
            name: row.try_get("name").unwrap_or_default(),
            setting: row.try_get("setting").unwrap_or_default(),
            unit: row.try_get("unit").unwrap_or_default(),
            short_desc: row.try_get("short_desc").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.setting, self.unit, self.short_desc]
    }

    fn headers() -> prettytable::Row {
        row!["name", "setting", "unit", "short_desc"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/db_settings.sql").to_string()
    }
}
