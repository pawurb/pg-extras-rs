use crate::{queries::shared::Query, PgStatsVersion};
use sqlx::postgres::types::Oid;
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone, serde::Serialize)]
pub struct NullIndexes {
    pub oid: Oid,
    pub index: String,
    pub index_size: String,
    pub unique: bool,
    pub indexed_column: String,
    pub table: String,
    pub null_frac: String,
    pub expected_saving: String,
    pub schema: String,
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

    fn read_file(_pg_statement_version: Option<PgStatsVersion>) -> String {
        include_str!("../sql/null_indexes.sql").to_string()
    }
}
