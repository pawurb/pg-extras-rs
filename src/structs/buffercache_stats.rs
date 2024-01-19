use crate::structs::shared::Tabular;
use postgres::Row;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[derive(Debug, Clone)]
pub struct BuffercacheStats {
    relname: String,
    buffered: String,
    buffer_percent: Decimal,
    percent_of_relation: Decimal,
}

impl Tabular for BuffercacheStats {
    const FILE_NAME: &'static str = "buffercache_stats";

    fn new(row: &Row) -> Self {
        Self {
            relname: row.get::<_, Option<String>>(0).unwrap_or_default(),
            buffered: row.get::<_, Option<String>>(1).unwrap_or_default(),
            buffer_percent: row.get::<_, Option<Decimal>>(2).unwrap_or(dec!(0)),
            percent_of_relation: row.get::<_, Option<Decimal>>(2).unwrap_or(dec!(0)),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.relname,
            self.buffered,
            self.buffer_percent.to_string(),
            self.percent_of_relation.to_string()
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "relname",
            "buffered",
            "buffer_percent",
            "percent_of_relation"
        ]
    }
}
