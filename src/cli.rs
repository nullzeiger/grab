use crate::error::{GrabError, Result};
use crate::models::App;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "grab",
    version = "0.1.0",
    author = "Ivan Guerreschi",
    about = "CLI tool to manage GitHub releases",
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add(AddArgs),
    Check(CheckArgs),
    Download,
    List,
    Remote(RemoteArgs),
    Remove(RemoveArgs),
    Search(SearchArgs),
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long)]
    pub owner: Option<String>,

    #[arg(short, long)]
    pub repo: Option<String>,

    #[arg(short, long, help = "Pattern to match release assets")]
    pub asset_pattern: Option<String>,

    #[arg(short, long, help = "Version flag")]
    pub version_flag: Option<String>,

    #[arg(short, long, help = "Enable interactive mode")]
    pub interactive: bool,
}

impl AddArgs {
    pub fn are_any_fields_missing(&self) -> bool {
        self.name.is_none() || self.owner.is_none() || self.repo.is_none()
    }

    pub fn into_app(self) -> Result<App> {
        App::new(
            self.name
                .ok_or_else(|| GrabError::InvalidInput("Name of the app is required".into()))?,
            self.owner
                .ok_or_else(|| GrabError::InvalidInput("Owner of the repo is required".into()))?,
            self.repo
                .ok_or_else(|| GrabError::InvalidInput("Name of the repo is required".into()))?,
            self.asset_pattern.ok_or_else(|| {
                GrabError::InvalidInput("Asset pattern of the repo is required".into())
            })?,
            self.version_flag
                .ok_or_else(|| GrabError::InvalidInput("Version flag is required".into()))?,
        )
    }
}

#[derive(Args, Debug)]
pub struct CheckArgs {
    #[arg(short, long)]
    pub download: bool,
}

#[derive(Args, Debug)]
pub struct RemoteArgs {
    #[arg(short, long)]
    pub file: Option<String>,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    #[arg(short, long)]
    pub index: usize,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    #[arg(short, long)]
    pub query: Option<String>,
}
