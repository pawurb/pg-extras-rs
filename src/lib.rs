use postgres::{Client, NoTls, Row};
use std::env;
pub mod structs;
pub use structs::all_locks::AllLocks;
pub use structs::bloat::Bloat;
pub use structs::blocking::Blocking;
pub use structs::buffercache_stats::BuffercacheStats;
pub use structs::buffercache_usage::BuffercacheUsage;
pub use structs::cache_hit::CacheHit;
pub use structs::calls::Calls;
pub use structs::connections::Connections;
pub use structs::db_settings::DbSetting;
pub use structs::duplicate_indexes::DuplicateIndexes;
pub use structs::extensions::Extensions;
pub use structs::index_cache_hit::IndexCacheHit;
pub use structs::index_scans::IndexScans;
pub use structs::index_size::IndexSize;
pub use structs::index_usage::IndexUsage;
pub use structs::indexes::Indexes;
pub use structs::locks::Locks;
pub use structs::long_running_queries::LongRunningQueries;
pub use structs::mandelbrot::Mandelbrot;
pub use structs::null_indexes::NullIndexes;
pub use structs::outliers::Outliers;
pub use structs::records_rank::RecordsRank;
pub use structs::seq_scans::SeqScans;
pub use structs::shared::{get_default_schema, Tabular};
pub use structs::ssl_used::SslUsed;
pub use structs::table_cache_hit::TableCacheHit;
pub use structs::table_index_scans::TableIndexScans;
pub use structs::table_indexes_size::TableIndexesSize;
pub use structs::table_size::TableSize;
pub use structs::tables::Tables;
pub use structs::total_index_size::TotalIndexSize;
pub use structs::total_table_size::TotalTableSize;
pub use structs::unused_indexes::UnusedIndexes;
pub use structs::vacuum_stats::VacuumStats;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

pub fn render_table<T: Tabular>(items: Vec<T>) {
    let mut table = Table::new();
    table.add_row(T::headers());

    for item in items {
        table.add_row(item.to_row());
    }
    table.printstd();
}

pub fn bloat() -> Vec<Bloat> {
    let query = read_file(Bloat::FILE_NAME);
    get_rows(query).iter().map(Bloat::new).collect()
}

pub fn blocking(limit: Option<String>) -> Vec<Blocking> {
    let limit = limit.unwrap_or("10".to_string());
    let query = read_file(Blocking::FILE_NAME).replace("%{limit}", limit.as_str());
    get_rows(&query).iter().map(Blocking::new).collect()
}

pub fn calls(limit: Option<String>) -> Vec<Calls> {
    let limit = limit.unwrap_or("10".to_string());
    let query = read_file("calls").replace("%{limit}", limit.as_str());
    get_rows(&query).iter().map(Calls::new).collect()
}

pub fn extensions() -> Vec<Extensions> {
    let query = read_file(Extensions::FILE_NAME);
    get_rows(query).iter().map(Extensions::new).collect()
}

pub fn table_cache_hit() -> Vec<TableCacheHit> {
    let query = read_file(TableCacheHit::FILE_NAME);
    get_rows(query).iter().map(TableCacheHit::new).collect()
}

pub fn tables(schema: Option<String>) -> Vec<Tables> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = read_file(Tables::FILE_NAME).replace("%{schema}", &schema_name);
    get_rows(&query).iter().map(Tables::new).collect()
}

pub fn index_cache_hit(schema: Option<String>) -> Vec<IndexCacheHit> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = read_file(IndexCacheHit::FILE_NAME).replace("%{schema}", &schema_name);
    get_rows(&query).iter().map(IndexCacheHit::new).collect()
}

pub fn indexes() -> Vec<Indexes> {
    let query = read_file(Indexes::FILE_NAME);
    get_rows(query).iter().map(Indexes::new).collect()
}

pub fn index_size() -> Vec<IndexSize> {
    let query = read_file(IndexSize::FILE_NAME);
    get_rows(query).iter().map(IndexSize::new).collect()
}

pub fn index_usage(schema: Option<String>) -> Vec<IndexUsage> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = read_file(IndexUsage::FILE_NAME).replace("%{schema}", &schema_name);
    get_rows(&query).iter().map(IndexUsage::new).collect()
}

pub fn index_scans(schema: Option<String>) -> Vec<IndexScans> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = read_file(IndexScans::FILE_NAME).replace("%{schema}", &schema_name);
    get_rows(&query).iter().map(IndexScans::new).collect()
}

pub fn null_indexes(min_relation_size_mb: Option<String>) -> Vec<NullIndexes> {
    let min_relation_size_mb = min_relation_size_mb.unwrap_or("0".to_string());
    let query =
        read_file(NullIndexes::FILE_NAME).replace("%{min_relation_size_mb}", &min_relation_size_mb);
    get_rows(&query).iter().map(NullIndexes::new).collect()
}

pub fn locks() -> Vec<Locks> {
    let query = read_file(Locks::FILE_NAME);
    get_rows(query).iter().map(Locks::new).collect()
}

pub fn all_locks() -> Vec<AllLocks> {
    let query = read_file(AllLocks::FILE_NAME);
    get_rows(query).iter().map(AllLocks::new).collect()
}

pub fn long_running_queries() -> Vec<LongRunningQueries> {
    let query = read_file(LongRunningQueries::FILE_NAME);
    get_rows(query)
        .iter()
        .map(LongRunningQueries::new)
        .collect()
}

pub fn mandelbrot() -> Vec<Mandelbrot> {
    let query = read_file(Mandelbrot::FILE_NAME);
    get_rows(query).iter().map(Mandelbrot::new).collect()
}

pub fn outliers() -> Vec<Outliers> {
    let query = read_file(Outliers::FILE_NAME);
    get_rows(query).iter().map(Outliers::new).collect()
}

pub fn records_rank(schema: Option<String>) -> Vec<RecordsRank> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = read_file(RecordsRank::FILE_NAME).replace("%{schema}", schema_name.as_str());
    get_rows(&query).iter().map(RecordsRank::new).collect()
}

pub fn seq_scans(schema: Option<String>) -> Vec<SeqScans> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = read_file(SeqScans::FILE_NAME).replace("%{schema}", schema_name.as_str());
    get_rows(&query).iter().map(SeqScans::new).collect()
}

pub fn table_index_scans(schema: Option<String>) -> Vec<TableIndexScans> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = read_file(TableIndexScans::FILE_NAME).replace("%{schema}", schema_name.as_str());
    get_rows(&query).iter().map(TableIndexScans::new).collect()
}

pub fn table_indexes_size(schema: Option<String>) -> Vec<TableIndexesSize> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = read_file(TableIndexesSize::FILE_NAME).replace("%{schema}", schema_name.as_str());
    get_rows(&query).iter().map(TableIndexesSize::new).collect()
}

pub fn table_size() -> Vec<TableSize> {
    let query = read_file(TableSize::FILE_NAME);
    get_rows(query).iter().map(TableSize::new).collect()
}

pub fn total_index_size() -> Vec<TotalIndexSize> {
    let query = read_file(TotalIndexSize::FILE_NAME);
    get_rows(query).iter().map(TotalIndexSize::new).collect()
}

pub fn total_table_size() -> Vec<TotalTableSize> {
    let query = read_file(TotalTableSize::FILE_NAME);
    get_rows(query).iter().map(TotalTableSize::new).collect()
}

pub fn unused_indexes(schema: Option<String>) -> Vec<UnusedIndexes> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = read_file(UnusedIndexes::FILE_NAME).replace("%{schema}", schema_name.as_str());
    get_rows(&query).iter().map(UnusedIndexes::new).collect()
}

pub fn duplicate_indexes() -> Vec<DuplicateIndexes> {
    let query = read_file(DuplicateIndexes::FILE_NAME);
    get_rows(query).iter().map(DuplicateIndexes::new).collect()
}

pub fn vacuum_stats() -> Vec<VacuumStats> {
    let query = read_file(VacuumStats::FILE_NAME);
    get_rows(query).iter().map(VacuumStats::new).collect()
}

pub fn buffercache_stats() -> Vec<BuffercacheStats> {
    let query = read_file(BuffercacheStats::FILE_NAME);
    get_rows(query).iter().map(BuffercacheStats::new).collect()
}

pub fn buffercache_usage() -> Vec<BuffercacheUsage> {
    let query = read_file(BuffercacheUsage::FILE_NAME);
    get_rows(query).iter().map(BuffercacheUsage::new).collect()
}

pub fn ssl_used() -> Vec<SslUsed> {
    let query = read_file(SslUsed::FILE_NAME);
    get_rows(query).iter().map(SslUsed::new).collect()
}

pub fn connections() -> Vec<Connections> {
    let query = read_file(Connections::FILE_NAME);
    get_rows(query).iter().map(Connections::new).collect()
}

pub fn cache_hit(schema: Option<String>) -> Vec<CacheHit> {
    let schema_name = schema.unwrap_or(get_default_schema());
    let query = read_file(CacheHit::FILE_NAME).replace("%{schema}", schema_name.as_str());
    get_rows(&query).iter().map(CacheHit::new).collect()
}

pub fn db_settings() -> Vec<DbSetting> {
    let query = read_file("db_settings");
    get_rows(query).iter().map(DbSetting::new).collect()
}

pub fn read_file(filename: &str) -> &'static str {
    match filename {
        "cache_hit" => include_str!("queries/cache_hit.sql"),
        "bloat" => include_str!("queries/bloat.sql"),
        "blocking" => include_str!("queries/blocking.sql"),
        "calls" => include_str!("queries/calls.sql"),
        "extensions" => include_str!("queries/extensions.sql"),
        "table_cache_hit" => include_str!("queries/table_cache_hit.sql"),
        "tables" => include_str!("queries/tables.sql"),
        "index_cache_hit" => include_str!("queries/index_cache_hit.sql"),
        "indexes" => include_str!("queries/indexes.sql"),
        "index_size" => include_str!("queries/index_size.sql"),
        "index_usage" => include_str!("queries/index_usage.sql"),
        "index_scans" => include_str!("queries/index_scans.sql"),
        "null_indexes" => include_str!("queries/null_indexes.sql"),
        "locks" => include_str!("queries/locks.sql"),
        "all_locks" => include_str!("queries/all_locks.sql"),
        "long_running_queries" => include_str!("queries/long_running_queries.sql"),
        "mandelbrot" => include_str!("queries/mandelbrot.sql"),
        "outliers" => include_str!("queries/outliers.sql"),
        "records_rank" => include_str!("queries/records_rank.sql"),
        "seq_scans" => include_str!("queries/seq_scans.sql"),
        "table_index_scans" => include_str!("queries/table_index_scans.sql"),
        "table_indexes_size" => include_str!("queries/table_indexes_size.sql"),
        "table_size" => include_str!("queries/table_size.sql"),
        "total_index_size" => include_str!("queries/total_index_size.sql"),
        "total_table_size" => include_str!("queries/total_table_size.sql"),
        "unused_indexes" => include_str!("queries/unused_indexes.sql"),
        "duplicate_indexes" => include_str!("queries/duplicate_indexes.sql"),
        "vacuum_stats" => include_str!("queries/vacuum_stats.sql"),
        "buffercache_stats" => include_str!("queries/buffercache_stats.sql"),
        "buffercache_usage" => include_str!("queries/buffercache_usage.sql"),
        "ssl_used" => include_str!("queries/ssl_used.sql"),
        "connections" => include_str!("queries/connections.sql"),
        _ => panic!("Unknown file: {}", filename),
    }
}

fn get_rows(query: &str) -> Vec<Row> {
    connection()
        .query(query, &[])
        .unwrap_or_else(|_| Vec::new())
}

fn connection() -> Client {
    let database_url = env::var("PG_EXTRAS_DATABASE_URL").unwrap_or_else(|_| {
        env::var("DATABASE_URL").expect(
            "Both $DATABASE_URL and $PG_EXTRAS_DATABASE_URL are not set",
        )
    });

    Client::connect(&database_url, NoTls).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        render_table(cache_hit(None));
        render_table(bloat());
        render_table(blocking(None));
        render_table(calls(None));
        render_table(extensions());
        render_table(table_cache_hit());
        render_table(tables(None));
        render_table(index_cache_hit(None));
        render_table(indexes());
        render_table(index_size());
        render_table(index_usage(None));
        render_table(index_scans(None));
        render_table(null_indexes(None));
        render_table(locks());
        render_table(all_locks());
        render_table(long_running_queries());
        render_table(mandelbrot());
        render_table(outliers());
        render_table(records_rank(None));
        render_table(seq_scans(None));
        render_table(table_index_scans(None));
        render_table(table_indexes_size(None));
        render_table(table_size());
        render_table(total_index_size());
        render_table(total_table_size());
        render_table(unused_indexes(None));
        render_table(duplicate_indexes());
        render_table(vacuum_stats());
        render_table(buffercache_stats());
        render_table(buffercache_usage());
        render_table(ssl_used());
        render_table(connections());
    }
}
