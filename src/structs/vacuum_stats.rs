use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug)]
pub struct VacuumStats {
    schema: String,
    table: String,
    last_vacuum: String,
    last_autovacuum: String,
    rowcount: String,
    dead_rowcount: String,
    autovacuum_threshold: String,
    expect_autovacuum: String,
}

impl Tabular for VacuumStats {
    const FILE_NAME: &'static str = "vacuum_stats";

    fn new(row: &Row) -> Self {
        VacuumStats {
            schema: row.get::<_, Option<String>>(0).unwrap_or_default(),
            table: row.get::<_, Option<String>>(1).unwrap_or_default(),
            last_vacuum: row.get::<_, Option<String>>(2).unwrap_or_default(),
            last_autovacuum: row.get::<_, Option<String>>(3).unwrap_or_default(),
            rowcount: row.get::<_, Option<String>>(4).unwrap_or_default(),
            dead_rowcount: row.get::<_, Option<String>>(4).unwrap_or_default(),
            autovacuum_threshold: row.get::<_, Option<String>>(4).unwrap_or_default(),
            expect_autovacuum: row.get::<_, Option<String>>(7).unwrap_or_default(),
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
}
