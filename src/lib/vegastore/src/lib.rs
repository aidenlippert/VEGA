use anyhow::Result;
use std::sync::Arc;
use vegagate::VegaGate;
use hex;

pub struct VegaStore {
    gate: Arc<VegaGate>,
}

impl VegaStore {
    pub fn new(gate: Arc<VegaGate>) -> Result<Self> {
        Ok(VegaStore { gate })
    }

    pub async fn store(&self, data: &[u8], _key: &str) -> Result<String> {
        Ok(format!("cid:{}", hex::encode(data)))
    }

    pub async fn retrieve(&self, _cid: &str, _key: &str) -> Result<Vec<u8>> {
        Ok(vec![0u8; 32]) // Placeholder
    }

    pub fn did(&self) -> &str {
        self.gate.did()  // Use VegaGate's public did() method
    }
}
