use clap::{Parser, Subcommand};
use pg_extras::{
    all_locks, bloat, blocking, buffercache_stats, buffercache_usage, cache_hit, calls,
    connections, db_settings, duplicate_indexes, extensions, index_cache_hit, index_scans,
    index_size, index_usage, indexes, locks, long_running_queries, mandelbrot, null_indexes,
    outliers, records_rank, render_table, seq_scans, ssl_used, table_cache_hit, table_index_scans,
    table_indexes_size, table_size, tables, total_index_size, total_table_size, unused_indexes,
    vacuum_stats, PgExtrasError,
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
    AllLocks(EmptyArgs),
    Bloat(EmptyArgs),
    Blocking(EmptyArgs),
    BuffercacheStats(EmptyArgs),
    BuffercacheUsage(EmptyArgs),
    CacheHit(EmptyArgs),
    Calls(EmptyArgs),
    Connections(EmptyArgs),
    DbSettings(EmptyArgs),
    DuplicateIndexes(EmptyArgs),
    Extensions(EmptyArgs),
    IndexCacheHit(EmptyArgs),
    IndexScans(EmptyArgs),
    IndexSize(EmptyArgs),
    IndexUsage(EmptyArgs),
    Indexes(EmptyArgs),
    Locks(EmptyArgs),
    LongRunningQueries(EmptyArgs),
    NullIndexes(EmptyArgs),
    Outliers(EmptyArgs),
    Mandelbrot(EmptyArgs),
    RecordsRank(EmptyArgs),
    SeqScans(EmptyArgs),
    SslUsed(EmptyArgs),
    TableCacheHit(EmptyArgs),
    TableIndexScans(EmptyArgs),
    TableIndexesSize(EmptyArgs),
    TableSize(EmptyArgs),
    Tables(EmptyArgs),
    TotalIndexSize(EmptyArgs),
    TotalTableSize(EmptyArgs),
    UnusedIndexes(EmptyArgs),
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

async fn execute() -> Result<(), PgExtrasError> {
    let args = PgExtrasArgs::parse();

    match args.cmd {
        PgSubcommand::AllLocks(_args) => {
            render_table(all_locks().await?);
        }
        PgSubcommand::Bloat(_args) => {
            render_table(bloat().await?);
        }
        PgSubcommand::Blocking(_args) => {
            render_table(blocking(None).await?);
        }
        PgSubcommand::BuffercacheStats(_args) => {
            render_table(buffercache_stats().await?);
        }
        PgSubcommand::BuffercacheUsage(_args) => {
            render_table(buffercache_usage().await?);
        }
        PgSubcommand::CacheHit(_args) => {
            render_table(cache_hit(None).await?);
        }
        PgSubcommand::Calls(_args) => {
            render_table(calls(None).await?);
        }
        PgSubcommand::Connections(_args) => {
            render_table(connections().await?);
        }
        PgSubcommand::DbSettings(_args) => {
            render_table(db_settings().await?);
        }
        PgSubcommand::DuplicateIndexes(_args) => {
            render_table(duplicate_indexes().await?);
        }
        PgSubcommand::Extensions(_args) => {
            render_table(extensions().await?);
        }
        PgSubcommand::IndexCacheHit(_args) => {
            render_table(index_cache_hit(None).await?);
        }
        PgSubcommand::IndexScans(_args) => {
            render_table(index_scans(None).await?);
        }
        PgSubcommand::IndexSize(_args) => {
            render_table(index_size().await?);
        }
        PgSubcommand::IndexUsage(_args) => {
            render_table(index_usage(None).await?);
        }
        PgSubcommand::Indexes(_args) => {
            render_table(indexes().await?);
        }
        PgSubcommand::Locks(_args) => {
            render_table(locks().await?);
        }
        PgSubcommand::LongRunningQueries(_args) => {
            render_table(long_running_queries().await?);
        }
        PgSubcommand::NullIndexes(_args) => {
            render_table(null_indexes(None).await?);
        }
        PgSubcommand::Outliers(_args) => {
            render_table(outliers().await?);
        }
        PgSubcommand::Mandelbrot(_args) => {
            render_table(mandelbrot().await?);
        }
        PgSubcommand::RecordsRank(_args) => {
            render_table(records_rank(None).await?);
        }
        PgSubcommand::SeqScans(_args) => {
            render_table(seq_scans(None).await?);
        }
        PgSubcommand::SslUsed(_args) => {
            render_table(ssl_used().await?);
        }
        PgSubcommand::TableCacheHit(_args) => {
            render_table(table_cache_hit().await?);
        }
        PgSubcommand::TableIndexScans(_args) => {
            render_table(table_index_scans(None).await?);
        }
        PgSubcommand::TableIndexesSize(_args) => {
            render_table(table_indexes_size(None).await?);
        }
        PgSubcommand::TableSize(_args) => {
            render_table(table_size().await?);
        }
        PgSubcommand::Tables(_args) => {
            render_table(tables(None).await?);
        }
        PgSubcommand::TotalIndexSize(_args) => {
            render_table(total_index_size().await?);
        }
        PgSubcommand::TotalTableSize(_args) => {
            render_table(total_table_size().await?);
        }
        PgSubcommand::UnusedIndexes(_args) => {
            render_table(unused_indexes(None).await?);
        }
        PgSubcommand::VacuumStats(_args) => {
            render_table(vacuum_stats().await?);
        }
    }

    Ok(())
}
