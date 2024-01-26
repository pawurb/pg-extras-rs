use crate::queries::shared::Query;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct VacuumStats {
    pub schema: String,
    pub table: String,
    pub last_vacuum: String,
    pub last_autovacuum: String,
    pub rowcount: String,
    pub dead_rowcount: String,
    pub autovacuum_threshold: String,
    pub expect_autovacuum: String,
}

impl Query for VacuumStats {
    fn new(row: &PgRow) -> Self {
        Self {
            schema: row.try_get("schema").unwrap_or_default(),
            table: row.try_get("table").unwrap_or_default(),
            last_vacuum: row.try_get("last_vacuum").unwrap_or_default(),
            last_autovacuum: row.try_get("last_autovacuum").unwrap_or_default(),
            rowcount: row.try_get("rowcount").unwrap_or_default(),
            dead_rowcount: row.try_get("dead_rowcount").unwrap_or_default(),
            autovacuum_threshold: row.try_get("autovacuum_threshold").unwrap_or_default(),
            expect_autovacuum: row.try_get("expect_autovacuum").unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.schema,
            self.table,
            self.last_vacuum,
            self.last_autovacuum,
            self.rowcount,
            self.dead_rowcount,
            self.autovacuum_threshold,
            self.expect_autovacuum
        ]
    }

    fn headers() -> prettytable::Row {
        row![
            "schema",
            "table",
            "last_vacuum",
            "last_autovacuum",
            "rowcount",
            "dead_rowcount",
            "autovacuum_threshold",
            "expect_autovacuum"
        ]
    }

    fn read_file() -> String {
        include_str!("../sql/vacuum_stats.sql").to_string()
    }
}
