use crate::structs::shared::Tabular;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct DbSetting {
    name: String,
    setting: String,
    unit: String,
    short_desc: String,
}

impl Tabular for DbSetting {
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
}
