use clap::Parser;
use stations_cli::{error::Error, Cli};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    // TODO handle this result
    cli.run().await
}
