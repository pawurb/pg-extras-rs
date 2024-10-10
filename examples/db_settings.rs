use pg_extras::{db_settings, render_table, PgExtrasError};

#[tokio::main]

async fn main() -> Result<(), PgExtrasError> {
    std::env::set_var(
        "PG_EXTRAS_DATABASE_URL",
        "postgres://postgres:secret@localhost:5432/pg-extras-rs-test",
    );

    let settings = db_settings().await?;
    render_table(settings);

    Ok(())
}
