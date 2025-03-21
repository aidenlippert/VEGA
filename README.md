cat << 'EOF' > README.md
# VEGA
Rust project for DID generation.

## Setup
- Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- LibTorch:
  1. Download `libtorch-shared-with-deps-2.4.1+cpu.zip` from [PyTorch](https://pytorch.org/get-started/locally/).
  2. Extract to `/home/user/libtorch`.
  3. Run:
     ```bash
     export LIBTORCH=/home/user/libtorch
     export LD_LIBRARY_PATH=$LIBTORCH/lib:$LD_LIBRARY_PATH
