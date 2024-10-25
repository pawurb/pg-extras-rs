use crate::{cache_hit, extensions, PgExtrasError};
use sqlx::types::BigDecimal;
use strum::{AsRefStr, EnumIter};

const PG_EXTRAS_TABLE_CACHE_HIT_MIN_EXPECTED: f64 = 0.985;

#[derive(Debug, EnumIter, AsRefStr)]
#[strum(serialize_all = "snake_case")]
enum Check {
    TableCacheHit,
    IndexCacheHit,
    SslUsed,
    UnusedIndexes,
    NullIndexes,
    Bloat,
    DuplicateIndexes,
    Outliers,
}

#[derive(Debug)]
pub struct CheckResult {
    pub ok: bool,
    pub message: String,
    pub check_name: String,
}

impl CheckResult {
    pub fn new(ok: bool, message: String, check_name: String) -> CheckResult {
        CheckResult {
            ok,
            message,
            check_name,
        }
    }
}

pub struct Diagnose;

impl Diagnose {
    pub async fn run() -> Result<Vec<CheckResult>, PgExtrasError>  {
        let mut checks = vec![
            Check::TableCacheHit,
            Check::IndexCacheHit,
            Check::UnusedIndexes,
            Check::NullIndexes,
            Check::Bloat,
            Check::DuplicateIndexes,
        ];

        let extensions_data = extensions().await?;

        if extensions_data.iter().any(|e| e.name == "sslinfo" && !e.installed_version.is_empty()) {
            checks.push(Check::SslUsed);
        }

        if extensions_data.iter().any(|e| e.name == "pg_stat_statements" && !e.installed_version.is_empty()) {
            checks.push(Check::Outliers);
        }

        let mut results = Vec::new();

        // run checks sequentially
        for check in checks {
            let result = Diagnose::run_check(check).await?;
            results.push(result);
        }

        Ok(results)
    }

    async fn run_check(check: Check) -> Result<CheckResult, PgExtrasError> {
        match check {
            Check::TableCacheHit => Diagnose::table_cache_hit().await,
            _ => Ok(CheckResult::new(false, "Not implemented".to_string(), "NotImplemented".to_string())),
        }
    }

    async fn table_cache_hit() -> Result<CheckResult, PgExtrasError> {
        let min_expected = BigDecimal::try_from(PG_EXTRAS_TABLE_CACHE_HIT_MIN_EXPECTED).unwrap();

        let cache_hit = cache_hit(None).await?;

        let table_cache_hit = cache_hit.iter().find(|item| item.name == "table hit rate");

        if let Some(table_hit_rate) = table_cache_hit {
            let ok = table_hit_rate.ratio >= min_expected;
            let message = format!("Table cache hit rate is {:.2}%", table_hit_rate.ratio);

            Ok(CheckResult::new(ok, message, format!("{}", Check::TableCacheHit.as_ref())))
        } else {
            Ok(CheckResult::new(false, "Table cache hit rate not found".to_string(), "TableCacheHit".to_string()))
        }

    }
}