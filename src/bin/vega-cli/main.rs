use anyhow::Result;
use clap::{Parser, Subcommand};
use vegastar::VegaStar;
use std::sync::Arc;
use vegagate::VegaGate;
use vegastore::VegaStore;

#[derive(Parser)]
#[command(name = "vega-cli", about = "Vega CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateDid { shard_id: u64, passphrase: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::CreateDid { shard_id, passphrase } => {
            let star = VegaStar::load_or_create(shard_id, &passphrase).await?;
            let gate = VegaGate::new(star)?;
            let store = VegaStore::new(Arc::new(gate))?;
            println!("Created DID: {}", store.did());
            Ok(())
        }
    }
}
