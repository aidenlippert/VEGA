#!/bin/bash
echo "Initializing VEGA environment..."
cargo build
cd frontend && yarn init -y && yarn install
echo "VEGA environment activated!"
