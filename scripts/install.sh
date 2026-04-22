#!/usr/bin/env bash
set -euo pipefail
echo ""
echo "  Installing OpenPRIME..."
echo ""
if ! command -v cargo &>/dev/null; then
    echo "  Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi
echo "  Building OpenPRIME from source..."
cargo install --git https://github.com/your-org/OpenPRIME prime-cli
echo ""
echo "  Done! Run: prime init && prime start"
echo ""
