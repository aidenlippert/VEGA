use anyhow::Result;
use vegastar::VegaStar;
use vegagate::VegaGate;
use vegastore::VegaStore;
use std::sync::Arc;

#[tokio::test]
async fn test_integration() -> Result<()> {
    let star = VegaStar::new()?;
    let gate = VegaGate::new(star)?;  // Removed & to pass by value
    let store = VegaStore::new(Arc::new(gate))?;
    let cid = store.store(b"test data", "key").await?;
    let retrieved = store.retrieve(&cid, "key").await?;
    assert_eq!(retrieved, vec![0u8; 32]);
    Ok(())
}
