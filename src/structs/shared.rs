use sqlx::postgres::{types::PgInterval, PgRow};
use std::env;

pub trait Tabular {
    fn new(row: &PgRow) -> Self;
    fn to_row(&self) -> prettytable::Row;
    fn headers() -> prettytable::Row;
}

pub fn get_default_interval() -> PgInterval {
    PgInterval {
        microseconds: 0,
        days: 0,
        months: 0,
    }
}

pub fn get_default_schema() -> String {
    env::var("PG_EXTRAS_SCHEMA").unwrap_or("public".to_string())
}
