use clap::Parser;
use stations_cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    // TODO handle this result
    let result = cli.run().await;
}
