use crate::size_parser::to_bytes;
use crate::{bloat, cache_hit, extensions, null_indexes, ssl_used, unused_indexes, Extensions, PgExtrasError};
use sqlx::types::BigDecimal;

const TABLE_CACHE_HIT_MIN: f32 = 0.985;
const INDEX_CACHE_HIT_MIN: f32 = 0.985;
const UNUSED_INDEXES_MIN_SIZE_BYTES: u64 = 1_000_000; // 1 MB
const NULL_INDEXES_MIN_SIZE_MB: &str = "1"; // 1 MB
const NULL_MIN_NULL_FRAC_PERCENT: f64 = 50.0; // 50%
const BLOAT_MIN_VALUE: f64 = 10.0;

#[derive(Debug)]
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
            Check::UnusedIndexes => Self::unused_index().await,
            Check::NullIndexes => Self::null_index().await,
            Check::Bloat => Self::bloat().await,
            Check::SslUsed => Self::ssl_used().await,
            _ => Ok(CheckResult::new(true, "Not implemented".to_string(), stringify!(check).to_string())),
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
            Ok(CheckResult::new(ok, message, stringify!(table_cache_hit).to_string()))
        } else {
            Ok(CheckResult::new(false, "Table cache hit rate not found".to_string(), stringify!(table_cache_hit).to_string()))
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
            Ok(CheckResult::new(ok, message, stringify!(index_cache_hit).to_string()))
        } else {
            Ok(CheckResult::new(false, "Index cache hit rate not found".to_string(), stringify!(index_cache_hit).to_string()))
        }
    }

    async fn ssl_used() -> Result<CheckResult, PgExtrasError> {
        if let Some(ssl_conn) = ssl_used().await?.first() {
            let message = if ssl_conn.ssl_used {
                "Database client is using a secure SSL connection."
            } else {
                "Database client is using an unencrypted connection."
            };
            return Ok(CheckResult::new(ssl_conn.ssl_used, message.to_string(), stringify!(ssl_used).to_string()));
        }
        Ok(CheckResult::new(false, "Unable to get connection information.".to_string(), stringify!(ssl_used).to_string()))
    }

    async fn unused_index() -> Result<CheckResult, PgExtrasError> {
        let indexes = unused_indexes(None).await?
            .into_iter()
            .filter(|i| to_bytes(&i.index_size).unwrap_or(0) >= UNUSED_INDEXES_MIN_SIZE_BYTES)
            .collect::<Vec<_>>();

        if indexes.is_empty() {
            return Ok(CheckResult::new(true, "No unused indexes detected.".to_string(), stringify!(unused_indexes).to_string()))
        }

        let print_indexes = indexes.iter()
            .map(|i| format!("'{}' on '{}' size {}", i.index, i.table, i.index_size))
            .collect::<Vec<_>>()
            .join(",\n");

        Ok(CheckResult::new(false, format!("Unused indexes detected:\n{}", print_indexes), stringify!(unused_indexes).to_string()))
    }

    async fn null_index() -> Result<CheckResult, PgExtrasError> {
        let indexes = null_indexes(Some(NULL_INDEXES_MIN_SIZE_MB.to_string())).await?
            .into_iter()
            .filter(|i|
                i.null_frac.trim_end_matches('%')
                .parse::<f64>()
                .unwrap_or(0.0) >= NULL_MIN_NULL_FRAC_PERCENT)
            .collect::<Vec<_>>();

        if indexes.is_empty() {
            return Ok(CheckResult::new(true, "No null indexes detected.".to_string(), stringify!(null_indexes).to_string()))
        }

        let print_indexes = indexes.iter()
            .map(|i| format!("'{}' size {} null values fraction {}", i.index, i.index_size, i.null_frac))
            .collect::<Vec<_>>()
            .join(",\n");

        Ok(CheckResult::new(false, format!("Null indexes detected:\n{}", print_indexes), stringify!(null_index).to_string()))
    }

    async fn bloat() -> Result<CheckResult, PgExtrasError> {
        let bloat_data = bloat().await?
            .into_iter()
            .filter(|b| b.bloat >= BigDecimal::try_from(BLOAT_MIN_VALUE).unwrap())
            .collect::<Vec<_>>();

        if bloat_data.is_empty() {
            return Ok(CheckResult::new(true, "No bloat detected.".to_string(), stringify!(bloat).to_string()))
        }

        let print_bloat = bloat_data.iter()
            .map(|b| format!("'{}' bloat {} waste {}", b.object_name, b.bloat, b.waste))
            .collect::<Vec<_>>()
            .join(",\n");

        Ok(CheckResult::new(false, format!("Bloat detected:\n{}", print_bloat), stringify!(bloat).to_string()))
    }
}