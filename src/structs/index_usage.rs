use crate::Tabular;
use postgres::Row;

pub struct IndexUsage {
    relname: String,
    percent_of_times_index_used: String,
    rows_in_table: i64,
}

impl Tabular for IndexUsage {
    const FILE_NAME: &'static str = "index_usage";

    fn new(row: &Row) -> Self {
        IndexUsage {
            relname: row.get::<_, Option<String>>(0).unwrap_or_default(),
            percent_of_times_index_used: row.get::<_, Option<String>>(1).unwrap_or_default(),
            rows_in_table: row.get::<_, Option<i64>>(2).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.relname,
            self.percent_of_times_index_used,
            self.rows_in_table.to_string()
        ]
    }

    fn headers() -> prettytable::Row {
        row!["relname", "percent_of_times_index_used", "rows_in_table"]
    }
}
