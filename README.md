# VEGA - The Ultimate Decentralized Ecosystem

VEGA is a cutting-edge decentralized platform with quantum-resistant identity, AI-driven networking, homomorphic storage, and a tokenized economy.

## Setup
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Build: `./scripts/build.sh`
3. Run: `cargo run --bin vega-node`

## Usage
- Create a DID: `cargo run --bin vega-cli -- create-did 1 mypassphrase`
- Store Data: `cargo run --bin vega-cli -- store-data 1 mypassphrase "Hello, VEGA!"`
