use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    
    #[command(subcommand)]
    command: Command,
}



#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Sync(SyncAction),
    Clone,
    Version,
}

impl Command {
    pub fn exec(self) -> Result<(), anyhow::Error> {
        match self {
            Command::Sync(s) => s.exec(),
            Command::Clone => todo!(),
            Command::Version => todo!(),
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct SyncAction {
    // todo fields
}

impl SyncAction {
    fn exec(self) -> Result<(), anyhow::Error> {
        // fetch github repo
        // load csv into stationrecord, either directly or via GeoJSON for which there's a tool in the repo

        todo!("this")
    }
}


struct StationRecord {
    // todo add fields from the CSV
    // todo use CSV parser crate to parse the csv file into Vec<StationRecord>
    // profit
}
