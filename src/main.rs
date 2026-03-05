use clap::Parser;
use color_eyre::eyre::Result;
use grab::cli::{Cli, Commands};
use grab::handlers;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    run().await
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add(args) => handlers::handle_add(args)?,
        Commands::Check(args) => handlers::handle_check(args).await?,
        Commands::Download => handlers::handle_download().await?,
        Commands::Remote(args) => handlers::handle_remote_download(args).await?,
        Commands::List => handlers::handle_list()?,
        Commands::Remove(args) => handlers::handle_remove(args)?,
        Commands::Search(args) => handlers::handle_search(args)?,
    }
    Ok(())
}
