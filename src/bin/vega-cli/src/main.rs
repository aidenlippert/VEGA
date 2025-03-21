use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::Value;
use vegastar::VegaStar;
use vegagate::VegaGate;
use vegastore::VegaStore;
use std::sync::Arc;

#[derive(Parser)]
#[command(version, about = "Vega CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateDid { shard_id: u64, passphrase: String },
    IssueCredential { subject: String, data: String },
    VerifyCredential { cid: String },
    StoreData { data: String, key: String },
    RetrieveData { cid: String, key: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateDid { shard_id, passphrase } => {
            let star = VegaStar::load_or_create(shard_id, &passphrase).await?;
            println!("Created DID: {}", star.did());
        }
        Commands::IssueCredential { subject, data } => {
            let star = VegaStar::new()?;
            let data_json: Value = serde_json::from_str(&data)?;
            let credential = star.issue_credential(&subject, data_json).await?;
            let id_str = credential.id.map_or("None".to_string(), |uri| uri.to_string());
            println!("Issued credential with ID: {}", id_str);
        }
        Commands::VerifyCredential { cid } => {
            let star = VegaStar::new()?;
            let gate = Arc::new(VegaGate::new(&star)?);
            let store = VegaStore::new(gate)?;
            let data: Vec<u8> = store.retrieve(&cid, "key").await?;
            println!("Verified credential data: {:?}", data);
        }
        Commands::StoreData { data, key } => {
            let star = VegaStar::new()?;
            let gate = Arc::new(VegaGate::new(&star)?);
            let store = VegaStore::new(gate)?;
            let cid = store.store(data.as_bytes(), &key).await?;
            println!("Stored data with CID: {}", cid);
        }
        Commands::RetrieveData { cid, key } => {
            let star = VegaStar::new()?;
            let gate = Arc::new(VegaGate::new(&star)?);
            let store = VegaStore::new(gate)?;
            let data: Vec<u8> = store.retrieve(&cid, &key).await?;
            println!("Retrieved data: {:?}", data);
        }
    }

    Ok(())
}
