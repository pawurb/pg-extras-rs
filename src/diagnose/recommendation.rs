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
        m
    };
}