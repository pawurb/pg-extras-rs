use crate::Tabular;
use postgres::Row;

pub struct DbSetting {
    name: String,
    setting: String,
    unit: String,
    short_desc: String,
}

impl Tabular for DbSetting {
    const FILE_NAME: &'static str = "db_settings";

    fn new(row: &Row) -> Self {
        DbSetting {
            name: row.get::<_, Option<String>>(0).unwrap_or_default(),
            setting: row.get::<_, Option<String>>(1).unwrap_or_default(),
            unit: row.get::<_, Option<String>>(2).unwrap_or_default(),
            short_desc: row.get::<_, Option<String>>(3).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.setting, self.unit, self.short_desc]
    }

    fn headers() -> prettytable::Row {
        row!["name", "setting", "unit", "short_desc"]
    }
}
