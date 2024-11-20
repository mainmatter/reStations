use clap::{Args, Parser, Subcommand};
use error::Error;
use sync::SyncAction;

pub mod error;
pub mod sync;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(flatten)]
    common: CommonArgs,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Args)]
pub struct CommonArgs {
    // Add common arguments here as they come up
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Sync(SyncAction),
}

impl Cli {
    pub async fn run(self) -> Result<(), Error> {
        match self.command {
            Command::Sync(sync_action) => sync_action.exec(self.common).await,
        }
    }
}
