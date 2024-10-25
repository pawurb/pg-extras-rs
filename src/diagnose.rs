use crate::{cache_hit, extensions, Extensions, PgExtrasError};
use sqlx::types::BigDecimal;
use strum::{AsRefStr, EnumIter};

const PG_EXTRAS_TABLE_CACHE_HIT_MIN_EXPECTED: f64 = 0.985;
const PG_EXTRAS_INDEX_CACHE_HIT_MIN_EXPECTED: f64 = 0.985;

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
    pub async fn run() -> Result<Vec<CheckResult>, PgExtrasError> {
        let mut checks = vec![
            Check::TableCacheHit,
            Check::IndexCacheHit,
            Check::UnusedIndexes,
            Check::NullIndexes,
            Check::Bloat,
            Check::DuplicateIndexes,
        ];

        let extensions_data = extensions().await?;

        if Diagnose::extension_enabled(&extensions_data, "sslinfo") {
            checks.push(Check::SslUsed);
        }

        if Diagnose::extension_enabled(&extensions_data, "pg_stat_statements") {
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

    fn extension_enabled(extensions_data: &Vec<Extensions>, extension_name: &str) -> bool {
        extensions_data
            .iter()
            .any(|e| e.name == extension_name && !e.installed_version.is_empty())
    }

    async fn run_check(check: Check) -> Result<CheckResult, PgExtrasError> {
        match check {
            Check::TableCacheHit => Diagnose::table_cache_hit().await,
            Check::IndexCacheHit => Diagnose::index_cache_hit().await,
            _ => Ok(CheckResult::new(
                false,
                "Not implemented".to_string(),
                "NotImplemented".to_string(),
            )),
        }
    }

    async fn table_cache_hit() -> Result<CheckResult, PgExtrasError> {
        let min_expected = BigDecimal::try_from(PG_EXTRAS_TABLE_CACHE_HIT_MIN_EXPECTED).unwrap();

        let cache_hit = cache_hit(None).await?;

        let table_cache_hit = cache_hit.iter().find(|item| item.name == "table hit rate");

        if let Some(table_hit_rate) = table_cache_hit {
            let ok = table_hit_rate.ratio >= min_expected;
            let message = if ok {
                format!("Table cache hit rate is correct: {:.4}", table_hit_rate.ratio)
            }
            else {
                format!("Table cache hit rate is too low: {:.4}", table_hit_rate.ratio)
            };

            Ok(CheckResult::new(
                ok,
                message,
                format!("{}", Check::TableCacheHit.as_ref()),
            ))
        } else {
            Ok(CheckResult::new(
                false,
                "Table cache hit rate not found".to_string(),
                format!("{}", Check::TableCacheHit.as_ref()),
            ))
        }
    }

    async fn index_cache_hit() -> Result<CheckResult, PgExtrasError> {
        let min_expected = BigDecimal::try_from(PG_EXTRAS_INDEX_CACHE_HIT_MIN_EXPECTED).unwrap();

        let cache_hit = cache_hit(None).await?;

        let index_cache_hit = cache_hit.iter().find(|item| item.name == "index hit rate");

        if let Some(index_hit_rate) = index_cache_hit {
            let ok = index_hit_rate.ratio >= min_expected;

            let message = if ok {
                format!("Index cache hit rate is correct: {:.4}", index_hit_rate.ratio)
            }
            else {
                format!("Index cache hit rate is too low: {:.4}", index_hit_rate.ratio)
            };

            Ok(CheckResult::new(
                ok,
                message,
                format!("{}", Check::IndexCacheHit.as_ref()),
            ))
        } else {
            Ok(CheckResult::new(
                false,
                "Index cache hit rate not found".to_string(),
                format!("{}", Check::IndexCacheHit.as_ref()),
            ))
        }
    }
}
