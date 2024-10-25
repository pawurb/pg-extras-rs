use crate::{cache_hit, extensions, ssl_used, Extensions, PgExtrasError};
use sqlx::types::BigDecimal;
use strum::AsRefStr;

const TABLE_CACHE_HIT_MIN: f64 = 0.985;
const INDEX_CACHE_HIT_MIN: f64 = 0.985;

#[derive(Debug, AsRefStr)]
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
    pub fn new(ok: bool, message: String, check_name: String) -> Self {
        Self { ok, message, check_name }
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

        if Self::extension_enabled(&extensions_data, "sslinfo") {
            checks.push(Check::SslUsed);
        }

        if Self::extension_enabled(&extensions_data, "pg_stat_statements") {
            checks.push(Check::Outliers);
        }

        let mut results = Vec::new();
        for check in checks {
            results.push(Self::run_check(check).await?);
        }

        Ok(results)
    }

    fn extension_enabled(extensions_data: &[Extensions], extension_name: &str) -> bool {
        extensions_data.iter().any(|e| e.name == extension_name && !e.installed_version.is_empty())
    }

    async fn run_check(check: Check) -> Result<CheckResult, PgExtrasError> {
        match check {
            Check::TableCacheHit => Self::table_cache_hit().await,
            Check::IndexCacheHit => Self::index_cache_hit().await,
            Check::SslUsed => Self::ssl_used().await,
            _ => Ok(CheckResult::new(true, "Not implemented".to_string(), check.as_ref().to_string())),
        }
    }

    async fn table_cache_hit() -> Result<CheckResult, PgExtrasError> {
        let min_expected = BigDecimal::try_from(TABLE_CACHE_HIT_MIN).unwrap();
        let cache_hit = cache_hit(None).await?;
        let table_cache_hit = cache_hit.iter().find(|item| item.name == "table hit rate");

        if let Some(table_hit_rate) = table_cache_hit {
            let ok = table_hit_rate.ratio >= min_expected;
            let message = if ok {
                format!("Table cache hit rate is correct: {:.4}", table_hit_rate.ratio)
            } else {
                format!("Table cache hit rate is too low: {:.4}", table_hit_rate.ratio)
            };
            Ok(CheckResult::new(ok, message, Check::TableCacheHit.as_ref().to_string()))
        } else {
            Ok(CheckResult::new(false, "Table cache hit rate not found".to_string(), Check::TableCacheHit.as_ref().to_string()))
        }
    }

    async fn index_cache_hit() -> Result<CheckResult, PgExtrasError> {
        let min_expected = BigDecimal::try_from(INDEX_CACHE_HIT_MIN).unwrap();
        let cache_hit = cache_hit(None).await?;
        let index_cache_hit = cache_hit.iter().find(|item| item.name == "index hit rate");

        if let Some(index_hit_rate) = index_cache_hit {
            let ok = index_hit_rate.ratio >= min_expected;
            let message = if ok {
                format!("Index cache hit rate is correct: {:.4}", index_hit_rate.ratio)
            } else {
                format!("Index cache hit rate is too low: {:.4}", index_hit_rate.ratio)
            };
            Ok(CheckResult::new(ok, message, Check::IndexCacheHit.as_ref().to_string()))
        } else {
            Ok(CheckResult::new(false, "Index cache hit rate not found".to_string(), Check::IndexCacheHit.as_ref().to_string()))
        }
    }

    async fn ssl_used() -> Result<CheckResult, PgExtrasError> {
        if let Some(ssl_conn) = ssl_used().await?.first() {
            let message = if ssl_conn.ssl_used {
                "Database client is using a secure SSL connection."
            } else {
                "Database client is using an unencrypted connection."
            };
            return Ok(CheckResult::new(ssl_conn.ssl_used, message.to_string(), Check::SslUsed.as_ref().to_string()));
        }
        Ok(CheckResult::new(false, "Unable to get connection information.".to_string(), Check::SslUsed.as_ref().to_string()))
    }
}