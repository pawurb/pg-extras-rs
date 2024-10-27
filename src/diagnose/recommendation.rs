use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::diagnose::run::Check;

lazy_static! {
    pub static ref Recommendations: HashMap<Check, (&'static str, Vec<&'static str>)> = {
        let mut m = HashMap::new();
        m.insert(
            Check::TableCacheHit,
            (
                "Cache hit rate is too low",
                vec![
                    "Review database settings: Consider comparing the database settings with ones recommended by PGTune and tweak values to improve performance.",
                    "Validate database specs: A low buffer cache hit ratio can be a sign that the Postgres instance is too small for the workload.",
                ],
            ),
        );
        m.insert(
            Check::IndexCacheHit,
            (
                "Cache hit rate is too low",
                vec![
                    "Review database settings: Consider comparing the database settings with ones recommended by PGTune and tweak values to improve performance.",
                    "Validate database specs: A low buffer cache hit ratio can be a sign that the Postgres instance is too small for the workload.",
                ],
            ),
        );
        m.insert(
            Check::UnusedIndexes,
            (
                "Remove unused indexes",
                vec![
                    "Consider eliminating indexes that are unused, which can impact the performance.",
                    "If the index is large, remember to use the CONCURRENTLY option when dropping it, to avoid exclusively blocking the whole related table."
                ],
            ),
        );
        m.insert(
            Check::NullIndexes,
            (
                "Optimize \"NULL\" indexes",
                vec![
                    "NULL values unnecessarily bloat the index size and slow down insert operations on a related table.",
                    "Consider replacing the index with a partial one that excludes NULL values.",
                ],
            ),
        );
        m.insert(
            Check::DuplicateIndexes,
            (
                "Remove duplicate indexes",
                vec![
                    "Consider removing the duplicate indexes to improve performance.",
                    "If the index is large, remember to use the CONCURRENTLY option when dropping it, to avoid exclusively blocking the whole related table."
                ],
            ),

        );
        m.insert(
            Check::SslUsed,
            (
                "SSL is not used",
                vec![
                    "Connecting to the database via an unencrypted connection is a critical security risk.",
                    "Consider enabling SSL to encrypt the connection between the client and the server.",
                ],
            ),
        );
        m.insert(
            Check::Bloat,
            (
                "Get rid of unnecessary bloat",
                vec![
                    "Review AUTOVACUUM settings: If it is misconfigured, it might result in your table consisting of mostly dead rows that are blocking the disk space and slowing down queries.",
                ],
            ),
        );
        m.insert(
            Check::Outliers,
            (
                "Add missing indexes",
                vec![
                    "Spot the queries that are consuming a lot of your database resources and are potentially missing an index.",
                    "Perform EXPLAIN ANALYZE and check if the query planner does Seq Scan on one of the tables.",
                ],
            ),
        );
        m
    };
}