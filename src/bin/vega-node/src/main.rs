use anyhow::Result;
use tokio;
use vegastar::VegaStar;
use vegagate::VegaGate;
use vegastore::VegaStore;
use veganet::VegaNet;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let star = VegaStar::new()?;
    let gate = VegaGate::new(&star)?;
    let _store = VegaStore::new(Arc::new(gate))?;

    let config = veganet::Config {
        listen_addr: "/ip4/0.0.0.0/tcp/0".to_string(),
    };
    let mut net = VegaNet::new(&config)?;
    net.run().await?;

    Ok(())
}
