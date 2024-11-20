use sqlx::postgres::{types::PgInterval, PgRow};
use std::env;

use crate::PgStatsVersion;

pub trait Query: serde::Serialize {
    fn new(row: &PgRow) -> Self;
    fn to_row(&self) -> prettytable::Row;
    fn headers() -> prettytable::Row;
    fn read_file(pg_statement_version: Option<PgStatsVersion>) -> String;
    fn description() -> String {
        let file_content = Self::read_file(None);
        let desc = file_content.lines().take(1).next().unwrap_or_default();
        extract_desc(desc)
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

fn extract_desc(desc: &str) -> String {
    if let (Some(start), Some(end)) = (desc.find("/*"), desc.find("*/")) {
        let extracted = &desc[start + 2..end];
        let mut trimmed = extracted.trim().to_string();
        if trimmed.ends_with('.') {
            trimmed.pop();
        }
        trimmed
    } else {
        desc.to_string()
    }
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
