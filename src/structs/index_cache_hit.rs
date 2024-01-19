use crate::structs::shared::Tabular;
use postgres::Row;

#[derive(Debug, Clone)]
pub struct IndexCacheHit {
    name: String,
    buffer_hits: i64,
    block_reads: i64,
    total_read: i64,
    ratio: String,
}

impl Tabular for IndexCacheHit {
    const FILE_NAME: &'static str = "index_cache_hit";

    fn new(row: &Row) -> Self {
        IndexCacheHit {
            name: row.get::<_, Option<String>>(0).unwrap_or_default(),
            buffer_hits: row.get::<_, Option<i64>>(1).unwrap_or_default(),
            block_reads: row.get::<_, Option<i64>>(2).unwrap_or_default(),
            total_read: row.get::<_, Option<i64>>(3).unwrap_or_default(),
            ratio: row.get::<_, Option<String>>(4).unwrap_or_default(),
        }
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.name,
            self.buffer_hits,
            self.block_reads,
            self.total_read,
            self.ratio
        ]
    }

    fn headers() -> prettytable::Row {
        row!["name", "buffer_hits", "block_reads", "total_read", "ratio"]
    }
}
