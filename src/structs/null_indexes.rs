use crate::structs::shared::Tabular;
use postgres::Row;

pub struct NullIndexes {
    oid: String,
    index: String,
    index_size: String,
    unique: String,
    indexed_column: String,
    table: String,
    null_frac: String,
    expected_saving: String,
    schema: String,
}

impl Tabular for NullIndexes {
    const FILE_NAME: &'static str = "null_indexes";

    fn new(row: &Row) -> Self {
        NullIndexes {
            oid: row.get::<_, Option<u32>>(0).unwrap_or_default().to_string(),
            index: row.get::<_, Option<String>>(1).unwrap_or_default(),
            index_size: row.get::<_, Option<String>>(2).unwrap_or_default(),
            unique: row
                .get::<_, Option<bool>>(3)
                .unwrap_or_default()
                .to_string(),
            indexed_column: row.get::<_, Option<String>>(4).unwrap_or_default(),
            table: row.get::<_, Option<String>>(5).unwrap_or_default(),
            null_frac: row.get::<_, Option<String>>(6).unwrap_or_default(),
            expected_saving: row.get::<_, Option<String>>(7).unwrap_or_default(),
            schema: row.get::<_, Option<String>>(8).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.oid,
            self.index,
            self.index_size,
            self.unique,
            self.indexed_column,
            self.table,
            self.null_frac,
            self.expected_saving,
            self.schema
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "oid",
            "index",
            "index_size",
            "unique",
            "indexed_column",
            "table",
            "null_frac",
            "expected_saving",
            "schema"
        ]
    }
}
