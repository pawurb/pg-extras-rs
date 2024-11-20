use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Extensions {
    pub name: String,
    pub default_version: String,
    pub installed_version: String,
    pub comment: String,
}

impl Query for Extensions {
    fn new(row: &PgRow) -> Self {
        Self {
            name: row.try_get("name").unwrap_or_default(),
            default_version: row.try_get("default_version").unwrap_or_default(),
            installed_version: row.try_get("installed_version").unwrap_or_default(),
            comment: row.try_get("comment").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.name,
            self.default_version,
            self.installed_version,
            self.comment
        ]
    }

    fn headers() -> prettytable::Row {
        row!["name", "default_version", "installed_version", "comment"]
    }

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/extensions.sql").to_string()
    }
}
