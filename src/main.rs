use clap::Parser;
use std::process;

mod app;
mod cli;
mod client;
mod compare_release_version;
mod error;
mod github_release;
mod github_version;
mod handlers;
mod models;
mod remote;
mod storage;
mod ui;

use cli::{Cli, Commands};
use error::Result;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(args) => handlers::handle_add(args),
        Commands::Check(args) => handlers::handle_check(args).await,
        Commands::Download => handlers::handle_download().await,
        Commands::Remote(args) => handlers::handle_remote_download(args).await,
        Commands::List => handlers::handle_list(),
        Commands::Remove(args) => handlers::handle_remove(args),
        Commands::Search(args) => handlers::handle_search(args),
    }
}
