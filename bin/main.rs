use clap::{Parser, Subcommand};
use pg_extras::diagnose::report::render_diagnose_report;
use pg_extras::{
    all_locks, bloat, blocking, buffercache_stats, buffercache_usage, cache_hit, calls,
    connections, db_settings, diagnose, duplicate_indexes, extensions, index_cache_hit,
    index_scans, index_size, index_usage, indexes, locks, long_running_queries, mandelbrot,
    null_indexes, outliers, pg_pool, records_rank, render_table, seq_scans, ssl_used,
    table_cache_hit, table_index_scans, table_indexes_size, table_size, tables, total_index_size,
    total_table_size, unused_indexes, vacuum_stats, AllLocks, Bloat, Blocking, BuffercacheStats,
    BuffercacheUsage, CacheHit, Calls, Connections, DbSettings, DuplicateIndexes, Extensions,
    IndexCacheHit, IndexScans, IndexSize, IndexUsage, Indexes, Locks, LongRunningQueries,
    Mandelbrot, NullIndexes, Outliers, PgExtrasError, Query, RecordsRank, SeqScans, SslUsed,
    TableCacheHit, TableIndexScans, TableIndexesSize, TableSize, Tables, TotalIndexSize,
    TotalTableSize, UnusedIndexes, VacuumStats,
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

#[cfg(feature = "web")]
use sqlx::PgPool;

#[derive(Subcommand, Debug)]
pub enum PgSubcommand {
    #[command(about = "Diagnose common database problems")]
    Diagnose(EmptyArgs),
    #[cfg(feature = "web")]
    #[command(about = "Start UI web server")]
    Web(EmptyArgs),
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

    let pool = pg_pool().await?;
    match args.cmd {
        PG::AllLocks(_args) => {
            render_table(all_locks(&pool).await?);
        }
        PG::Bloat(_args) => {
            render_table(bloat(&pool).await?);
        }
        PG::Blocking(_args) => {
            render_table(blocking(None, &pool).await?);
        }
        PG::BuffercacheStats(_args) => {
            render_table(buffercache_stats(&pool).await?);
        }
        PG::BuffercacheUsage(_args) => {
            render_table(buffercache_usage(&pool).await?);
        }
        PG::CacheHit(_args) => {
            render_table(cache_hit(None, &pool).await?);
        }
        PG::Calls(_args) => {
            render_table(calls(None, &pool).await?);
        }
        PG::Connections(_args) => {
            render_table(connections(&pool).await?);
        }
        PG::DbSettings(_args) => {
            render_table(db_settings(&pool).await?);
        }
        PG::Diagnose(_args) => {
            render_diagnose_report(diagnose(&pool).await?);
        }
        PG::DuplicateIndexes(_args) => {
            render_table(duplicate_indexes(&pool).await?);
        }
        PG::Extensions(_args) => {
            render_table(extensions(&pool).await?);
        }
        PG::IndexCacheHit(_args) => {
            render_table(index_cache_hit(None, &pool).await?);
        }
        PG::IndexScans(_args) => {
            render_table(index_scans(None, &pool).await?);
        }
        PG::IndexSize(_args) => {
            render_table(index_size(&pool).await?);
        }
        PG::IndexUsage(_args) => {
            render_table(index_usage(None, &pool).await?);
        }
        PG::Indexes(_args) => {
            render_table(indexes(&pool).await?);
        }
        PG::Locks(_args) => {
            render_table(locks(&pool).await?);
        }
        PG::LongRunningQueries(_args) => {
            render_table(long_running_queries(&pool).await?);
        }
        PG::NullIndexes(_args) => {
            render_table(null_indexes(None, &pool).await?);
        }
        PG::Outliers(_args) => {
            render_table(outliers(&pool).await?);
        }
        PG::Mandelbrot(_args) => {
            render_table(mandelbrot(&pool).await?);
        }
        PG::RecordsRank(_args) => {
            render_table(records_rank(None, &pool).await?);
        }
        PG::SeqScans(_args) => {
            render_table(seq_scans(None, &pool).await?);
        }
        PG::SslUsed(_args) => {
            render_table(ssl_used(&pool).await?);
        }
        PG::TableCacheHit(_args) => {
            render_table(table_cache_hit(&pool).await?);
        }
        PG::TableIndexScans(_args) => {
            render_table(table_index_scans(None, &pool).await?);
        }
        PG::TableIndexesSize(_args) => {
            render_table(table_indexes_size(None, &pool).await?);
        }
        PG::TableSize(_args) => {
            render_table(table_size(&pool).await?);
        }
        PG::Tables(_args) => {
            render_table(tables(None, &pool).await?);
        }
        PG::TotalIndexSize(_args) => {
            render_table(total_index_size(&pool).await?);
        }
        PG::TotalTableSize(_args) => {
            render_table(total_table_size(&pool).await?);
        }
        PG::UnusedIndexes(_args) => {
            render_table(unused_indexes(None, &pool).await?);
        }
        PG::VacuumStats(_args) => {
            render_table(vacuum_stats(&pool).await?);
        }
        #[cfg(feature = "web")]
        PG::Web(_args) => {
            start_web_server(pool).await?;
        }
    }

    Ok(())
}

#[cfg(feature = "web")]
async fn start_web_server(pool: PgPool) -> Result<(), PgExtrasError> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    if tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .is_err()
    {
        PgExtrasError::Other(format!("Port {} is already in use", port));
    }

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    let app = pg_extras::web::routes::app(pool).await;

    println!(
        "Available on http://{}/pg_extras",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
