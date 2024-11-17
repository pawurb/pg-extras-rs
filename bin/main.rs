use clap::{Parser, Subcommand};
use pg_extras::diagnose::report::render_diagnose_report;
use pg_extras::{
    all_locks, bloat, blocking, buffercache_stats, buffercache_usage, cache_hit, calls,
    connections, db_settings, diagnose, duplicate_indexes, extensions, index_cache_hit,
    index_scans, index_size, index_usage, indexes, locks, long_running_queries, mandelbrot,
    null_indexes, outliers, records_rank, render_table, seq_scans, ssl_used, table_cache_hit,
    table_index_scans, table_indexes_size, table_size, tables, total_index_size, total_table_size,
    unused_indexes, vacuum_stats, AllLocks, Bloat, Blocking, BuffercacheStats, BuffercacheUsage,
    CacheHit, Calls, Connections, DbSettings, DuplicateIndexes, Extensions, IndexCacheHit,
    IndexScans, IndexSize, IndexUsage, Indexes, Locks, LongRunningQueries, Mandelbrot, NullIndexes,
    Outliers, PgExtrasError, Query, RecordsRank, SeqScans, SslUsed, TableCacheHit, TableIndexScans,
    TableIndexesSize, TableSize, Tables, TotalIndexSize, TotalTableSize, UnusedIndexes,
    VacuumStats,
};

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "pg-extras: PostgreSQL database performance insights. Locks, index usage, buffer cache hit ratios, vacuum stats and more.

https://github.com/pawurb/pg-extras-rs"
)]
pub struct PgExtrasArgs {
    #[command(subcommand)]
    pub cmd: PgSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum PgSubcommand {
    #[command(about = "Diagnose common database problems")]
    Diagnose(EmptyArgs),
    #[command(about = &AllLocks::description())]
    AllLocks(EmptyArgs),
    #[command(about = &Bloat::description())]
    Bloat(EmptyArgs),
    #[command(about = &Blocking::description())]
    Blocking(EmptyArgs),
    #[command(about = &BuffercacheStats::description())]
    BuffercacheStats(EmptyArgs),
    #[command(about = &BuffercacheUsage::description())]
    BuffercacheUsage(EmptyArgs),
    #[command(about = &CacheHit::description())]
    CacheHit(EmptyArgs),
    #[command(about = &Calls::description())]
    Calls(EmptyArgs),
    #[command(about = &Connections::description())]
    Connections(EmptyArgs),
    #[command(about = &DbSettings::description())]
    DbSettings(EmptyArgs),
    #[command(about = &DuplicateIndexes::description())]
    DuplicateIndexes(EmptyArgs),
    #[command(about = &Extensions::description())]
    Extensions(EmptyArgs),
    #[command(about = &IndexCacheHit::description())]
    IndexCacheHit(EmptyArgs),
    #[command(about = &IndexScans::description())]
    IndexScans(EmptyArgs),
    #[command(about = &IndexSize::description())]
    IndexSize(EmptyArgs),
    #[command(about = &IndexUsage::description())]
    IndexUsage(EmptyArgs),
    #[command(about = &Indexes::description())]
    Indexes(EmptyArgs),
    #[command(about = &Locks::description())]
    Locks(EmptyArgs),
    #[command(about = &LongRunningQueries::description())]
    LongRunningQueries(EmptyArgs),
    #[command(about = &NullIndexes::description())]
    NullIndexes(EmptyArgs),
    #[command(about = &Outliers::description())]
    Outliers(EmptyArgs),
    #[command(about = &Mandelbrot::description())]
    Mandelbrot(EmptyArgs),
    #[command(about = &RecordsRank::description())]
    RecordsRank(EmptyArgs),
    #[command(about = &SeqScans::description())]
    SeqScans(EmptyArgs),
    #[command(about = &SslUsed::description())]
    SslUsed(EmptyArgs),
    #[command(about = &TableCacheHit::description())]
    TableCacheHit(EmptyArgs),
    #[command(about = &TableIndexScans::description())]
    TableIndexScans(EmptyArgs),
    #[command(about = &TableIndexesSize::description())]
    TableIndexesSize(EmptyArgs),
    #[command(about = &TableSize::description())]
    TableSize(EmptyArgs),
    #[command(about = &Tables::description())]
    Tables(EmptyArgs),
    #[command(about = &TotalIndexSize::description())]
    TotalIndexSize(EmptyArgs),
    #[command(about = &TotalTableSize::description())]
    TotalTableSize(EmptyArgs),
    #[command(about = &UnusedIndexes::description())]
    UnusedIndexes(EmptyArgs),
    #[command(about = &VacuumStats::description())]
    VacuumStats(EmptyArgs),
}

#[derive(Parser, Debug)]
pub struct EmptyArgs {}

#[tokio::main]
async fn main() {
    match execute().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

type PG = PgSubcommand;
async fn execute() -> Result<(), PgExtrasError> {
    let args = PgExtrasArgs::parse();

    match args.cmd {
        PG::AllLocks(_args) => {
            render_table(all_locks().await?);
        }
        PG::Bloat(_args) => {
            render_table(bloat().await?);
        }
        PG::Blocking(_args) => {
            render_table(blocking(None).await?);
        }
        PG::BuffercacheStats(_args) => {
            render_table(buffercache_stats().await?);
        }
        PG::BuffercacheUsage(_args) => {
            render_table(buffercache_usage().await?);
        }
        PG::CacheHit(_args) => {
            render_table(cache_hit(None).await?);
        }
        PG::Calls(_args) => {
            render_table(calls(None).await?);
        }
        PG::Connections(_args) => {
            render_table(connections().await?);
        }
        PG::DbSettings(_args) => {
            render_table(db_settings().await?);
        }
        PG::Diagnose(_args) => {
            render_diagnose_report(diagnose().await?);
        }
        PG::DuplicateIndexes(_args) => {
            render_table(duplicate_indexes().await?);
        }
        PG::Extensions(_args) => {
            render_table(extensions().await?);
        }
        PG::IndexCacheHit(_args) => {
            render_table(index_cache_hit(None).await?);
        }
        PG::IndexScans(_args) => {
            render_table(index_scans(None).await?);
        }
        PG::IndexSize(_args) => {
            render_table(index_size().await?);
        }
        PG::IndexUsage(_args) => {
            render_table(index_usage(None).await?);
        }
        PG::Indexes(_args) => {
            render_table(indexes().await?);
        }
        PG::Locks(_args) => {
            render_table(locks().await?);
        }
        PG::LongRunningQueries(_args) => {
            render_table(long_running_queries().await?);
        }
        PG::NullIndexes(_args) => {
            render_table(null_indexes(None).await?);
        }
        PG::Outliers(_args) => {
            render_table(outliers().await?);
        }
        PG::Mandelbrot(_args) => {
            render_table(mandelbrot().await?);
        }
        PG::RecordsRank(_args) => {
            render_table(records_rank(None).await?);
        }
        PG::SeqScans(_args) => {
            render_table(seq_scans(None).await?);
        }
        PG::SslUsed(_args) => {
            render_table(ssl_used().await?);
        }
        PG::TableCacheHit(_args) => {
            render_table(table_cache_hit().await?);
        }
        PG::TableIndexScans(_args) => {
            render_table(table_index_scans(None).await?);
        }
        PG::TableIndexesSize(_args) => {
            render_table(table_indexes_size(None).await?);
        }
        PG::TableSize(_args) => {
            render_table(table_size().await?);
        }
        PG::Tables(_args) => {
            render_table(tables(None).await?);
        }
        PG::TotalIndexSize(_args) => {
            render_table(total_index_size().await?);
        }
        PG::TotalTableSize(_args) => {
            render_table(total_table_size().await?);
        }
        PG::UnusedIndexes(_args) => {
            render_table(unused_indexes(None).await?);
        }
        PG::VacuumStats(_args) => {
            render_table(vacuum_stats().await?);
        }
    }

    Ok(())
}
