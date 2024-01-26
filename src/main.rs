use pg_extras::*;
use std::env;
use thiserror::Error;

#[tokio::main]
async fn main() -> Result<(), PgExtrasCmdError> {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    if let Some(command) = command {
        match &command[..] {
            "all_locks" => {
                render_table(all_locks().await?);
            }
            "bloat" => {
                render_table(bloat().await?);
            }
            "blocking" => {
                render_table(blocking(None).await?);
            }
            "buffercache_stats" => {
                render_table(buffercache_stats().await?);
            }
            "buffercache_usage" => {
                render_table(buffercache_usage().await?);
            }
            "cache_hit" => {
                render_table(cache_hit(None).await?);
            }
            "calls" => {
                render_table(calls(None).await?);
            }
            "connections" => {
                render_table(connections().await?);
            }
            "db_settings" => {
                render_table(db_settings().await?);
            }
            "duplicate_indexes" => {
                render_table(duplicate_indexes().await?);
            }
            "extensions" => {
                render_table(extensions().await?);
            }
            "index_cache_hit" => {
                render_table(index_cache_hit(None).await?);
            }
            "index_scans" => {
                render_table(index_scans(None).await?);
            }
            "index_size" => {
                render_table(index_size().await?);
            }
            "index_usage" => {
                render_table(index_usage(None).await?);
            }
            "indexes" => {
                render_table(indexes().await?);
            }
            "locks" => {
                render_table(locks().await?);
            }
            "long_running_queries" => {
                render_table(long_running_queries().await?);
            }
            "mandelbrot" => {
                render_table(mandelbrot().await?);
            }
            "null_indexes" => {
                render_table(null_indexes(None).await?);
            }
            "outliers" => {
                render_table(outliers().await?);
            }
            "records_rank" => {
                render_table(records_rank(None).await?);
            }
            "seq_scans" => {
                render_table(seq_scans(None).await?);
            }
            "ssl_used" => {
                render_table(ssl_used().await?);
            }
            "table_cache_hit" => {
                render_table(table_cache_hit().await?);
            }
            "table_index_scans" => {
                render_table(table_index_scans(None).await?);
            }
            "table_indexes_size" => {
                render_table(table_indexes_size(None).await?);
            }
            "table_size" => {
                render_table(table_size().await?);
            }
            "tables" => {
                render_table(tables(None).await?);
            }
            "total_index_size" => {
                render_table(total_index_size().await?);
            }
            "total_table_size" => {
                render_table(total_table_size().await?);
            }
            "unused_indexes" => {
                render_table(unused_indexes(None).await?);
            }
            "vacuum_stats" => {
                render_table(vacuum_stats().await?);
            }
            _ => {
                return Err(PgExtrasCmdError::UnknownCommand(command));
            }
        }
    }

    Ok(())
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PgExtrasCmdError {
    #[error("Unknown command")]
    UnknownCommand(String),
    #[error("Unknown pg-extras error")]
    Other(PgExtrasError),
}

impl From<pg_extras::PgExtrasError> for PgExtrasCmdError {
    fn from(error: pg_extras::PgExtrasError) -> Self {
        Self::Other(error)
    }
}