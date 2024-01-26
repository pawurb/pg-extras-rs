use crate::queries::shared::Query;
use sqlx::postgres::types::Oid;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct NullIndexes {
    oid: Oid,
    index: String,
    index_size: String,
    unique: bool,
    indexed_column: String,
    table: String,
    null_frac: String,
    expected_saving: String,
    schema: String,
}

impl Query for NullIndexes {
    fn new(row: &PgRow) -> Self {
        Self {
            oid: row.try_get("oid").unwrap_or_default(),
            index: row.try_get("index").unwrap_or_default(),
            index_size: row.try_get("index_size").unwrap_or_default(),
            unique: row.try_get("unique").unwrap_or_default(),
            indexed_column: row.try_get("indexed_column").unwrap_or_default(),
            table: row.try_get("table").unwrap_or_default(),
            null_frac: row.try_get("null_frac").unwrap_or_default(),
            expected_saving: row.try_get("expected_saving").unwrap_or_default(),
            schema: row.try_get("schema").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            format!("{:?}", self.oid),
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

    fn read_file() -> String {
        include_str!("../sql/null_indexes.sql").to_string()
    }
}
