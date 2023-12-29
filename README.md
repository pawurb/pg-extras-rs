# rust-pg-extras [![Latest Version](https://img.shields.io/crates/v/pg-extras.svg)](https://crates.io/crates/pg-extras)

Rust port of [Heroku PG Extras](https://github.com/heroku/heroku-pg-extras) with several additions and improvements. The goal of this project is to provide powerful insights into the PostgreSQL database for Rust apps that are not using the Heroku PostgreSQL plugin.

Queries can be used to obtain information about a Postgres instance, that may be useful when analyzing performance issues. This includes information about locks, index usage, buffer cache hit ratios and vacuum statistics. Ruby API enables developers to easily integrate the tool into e.g. automatic monitoring tasks.
