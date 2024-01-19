use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct BuffercacheUsage {
    relname: String,
    buffers: i64,
}

impl Tabular for BuffercacheUsage {
    const FILE_NAME: &'static str = "buffercache_usage";

    fn new(row: &Row) -> Self {
        Self {
            relname: row.get::<_, Option<String>>(0).unwrap_or_default(),
            buffers: row.get::<_, Option<i64>>(1).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.relname, self.buffers]
    }

    fn headers() -> prettytable::Row {
        row!["relname", "buffers"]
    }
}
