use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Extensions {
    name: String,
    default_version: String,
    installed_version: String,
    comment: String,
}

impl Tabular for Extensions {
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
}
