use pg_interval::Interval;
use postgres::{Client, NoTls, Row};
use std::env;

pub trait Tabular {
    fn new(row: &Row) -> Self;
    const FILE_NAME: &'static str;
    fn to_row(&self) -> prettytable::Row;
    fn headers() -> prettytable::Row;
}

pub fn get_default_interval() -> Interval {
    Interval::from_postgres("0 seconds").unwrap()
}

pub fn get_default_schema() -> String {
    env::var("PG_EXTRAS_SCHEMA").unwrap_or("public".to_string())
}

pub fn get_rows(query: &str) -> Vec<Row> {
    connection()
        .query(query, &[])
        .unwrap_or_else(|_| Vec::new())
}

fn connection() -> Client {
    let database_url = env::var("DATABASE_URL").expect("$DATABASE_URL is not set");
    Client::connect(&database_url, NoTls).unwrap()
}
