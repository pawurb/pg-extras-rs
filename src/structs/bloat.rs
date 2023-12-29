use crate::structs::shared::Tabular;
use postgres::Row;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub struct Bloat {
    typefield: String,
    schemaname: String,
    object_name: String,
    bloat: Decimal,
    waste: String,
}

impl Tabular for Bloat {
    const FILE_NAME: &'static str = "bloat";

    fn new(row: &Row) -> Self {
        Bloat {
            typefield: row.get::<_, Option<String>>(0).unwrap_or_default(),
            schemaname: row.get::<_, Option<String>>(1).unwrap_or_default(),
            object_name: row.get::<_, Option<String>>(2).unwrap_or_default(),
            bloat: row.get::<_, Option<Decimal>>(3).unwrap_or(dec!(0)),
            waste: row.get::<_, Option<String>>(4).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.typefield,
            self.schemaname,
            self.object_name,
            self.bloat,
            self.waste
        ]
    }

    fn headers() -> prettytable::Row {
        row!["type", "schemaname", "object_name", "bloat", "waste"]
    }
}
