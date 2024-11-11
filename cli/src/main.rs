use clap::Parser;
use stations_cli::Cli;

fn main() {
    let cmd = Cli::parse();

    dbg!(cmd);
}