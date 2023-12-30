use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug)]
pub struct Extensions {
    name: String,
    default_version: String,
    installed_version: String,
    comment: String,
}

impl Tabular for Extensions {
    const FILE_NAME: &'static str = "extensions";

    fn new(row: &Row) -> Self {
        Extensions {
            name: row.get::<_, Option<String>>(0).unwrap_or_default(),
            default_version: row.get::<_, Option<String>>(1).unwrap_or_default(),
            installed_version: row.get::<_, Option<String>>(2).unwrap_or_default(),
            comment: row.get::<_, Option<String>>(3).unwrap_or_default(),
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
