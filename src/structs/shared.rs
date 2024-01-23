use pg_interval::Interval;
use postgres::Row;
use std::env;

pub trait Tabular {
    fn new(row: &Row) -> Self;
    fn to_row(&self) -> prettytable::Row;
    fn headers() -> prettytable::Row;
}

pub fn get_default_interval() -> Interval {
    Interval::from_postgres("0 seconds").unwrap()
}

pub fn get_default_schema() -> String {
    env::var("PG_EXTRAS_SCHEMA").unwrap_or("public".to_string())
}
