mod api;
mod auth;
mod command_line_args;
mod db;
mod db_setup;
mod error;
mod pages;

mod http_server;
mod http_static;

mod structs;
mod util;

use clap::Parser;
use http_server::start_http_server;

use anyhow::{Context, Result};

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::command_line_args::CommandLineArgs;
use crate::db_setup::setup_database;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();

    let command_line_args = CommandLineArgs::parse();
    let sqlite_pool = setup_database(&command_line_args.database).await;

    // depending on the feature flags, the pool needs a mutable reference or not
    tokio::select! {
        res = start_http_server(sqlite_pool.clone()) => {
            res.context("http server crashed").unwrap();
        }
    }

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}
