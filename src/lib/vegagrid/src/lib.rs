use anyhow::Result;
use ipfs_api::{IpfsClient, IpfsApi};
use sodiumoxide::crypto::secretbox;
use tokio;

pub struct VegaGrid {
    ipfs: IpfsClient,
    encryption_key: secretbox::Key,
}

impl VegaGrid {
    pub async fn new() -> Result<Self> {
        sodiumoxide::init().map_err(|_| anyhow::anyhow!("Sodium init failed"))?;
        let ipfs = IpfsClient::default(); // Connects to localhost:5001 by default
        let encryption_key = secretbox::gen_key();
        Ok(VegaGrid { ipfs, encryption_key })
    }

    pub async fn store_file(&self, data: &[u8]) -> Result<String> {
        let nonce = secretbox::gen_nonce();
        let encrypted_data = secretbox::seal(data, &nonce, &self.encryption_key);
        let response = self.ipfs.add(std::io::Cursor::new(encrypted_data)).await?;
        println!("Stored encrypted file at CID: {}", response.hash);
        Ok(response.hash) // Return CID as String
    }

    pub async fn retrieve_file(&self, cid: &str) -> Result<Vec<u8>> {
        let encrypted_data = self.ipfs.cat(cid).await?.collect::<Vec<u8>>();
        let nonce = secretbox::gen_nonce(); // Placeholder; in production, store nonce with CID
        let decrypted_data = secretbox::open(&encrypted_data, &nonce, &self.encryption_key)
            .map_err(|_| anyhow::anyhow!("Decryption failed"))?;
        Ok(decrypted_data)
    }
}

#[tokio::test]
async fn test_store_and_retrieve() {
    let grid = VegaGrid::new().await.unwrap();
    let data = b"Hello, VEGA!";
    let cid = grid.store_file(data).await.unwrap();
    let retrieved = grid.retrieve_file(&cid).await.unwrap();
    assert_eq!(data.to_vec(), retrieved);
}
