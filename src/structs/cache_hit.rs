use crate::structs::shared::Tabular;
use postgres::Row;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[derive(Debug, Clone)]
pub struct CacheHit {
    name: String,
    ratio: Decimal,
}

impl Tabular for CacheHit {
    const FILE_NAME: &'static str = "cache_hit";

    fn new(row: &Row) -> Self {
        CacheHit {
            name: row.get::<_, Option<String>>(0).unwrap_or_default(),
            ratio: row.get::<_, Option<Decimal>>(1).unwrap_or(dec!(0)),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.ratio]
    }

    fn headers() -> prettytable::Row {
        row!["name", "ratio"]
    }
}
