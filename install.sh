#!/bin/bash

# Server OS Installation Script
# Installs everything from official crates.io

set -e

echo "🖥️  Server OS Installation"
echo "=========================="
echo ""

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "📦 Installing Server OS REPL..."
cargo build --release
cargo install --path .

echo ""
echo "📦 Installing tools from crates.io..."

# File manager (only yazi as requested)
echo "📁 Installing Yazi file manager..."
cargo install --locked yazi-fm yazi-cli

# System monitor
echo "📊 Installing bottom system monitor..."
cargo install bottom

# Network tools
echo "🌐 Installing bandwhich network monitor..."
cargo install bandwhich

echo "🌐 Installing trippy network diagnostics..."
cargo install trippy

echo "📊 Installing serie git graph viewer..."
cargo install serie

echo ""
echo "✅ Server OS installed successfully!"
echo ""
echo "🚀 Usage: os"
echo ""
echo "Available commands in REPL:"
echo "  :help     - Show all available commands"
echo "  :finder   - Launch Yazi file manager"
echo "  :system   - Launch bottom system monitor"
echo "  :network  - Launch bandwhich network monitor"
echo "  :trace    - Network diagnostics with trippy"
echo "  :git      - Launch serie git graph viewer"
echo "  :status   - Show system status"
echo "  :update   - Update Server OS to latest version"
echo "  :exit     - Exit the REPL"
echo ""
echo "Shell commands work directly without prefix"
echo ""
echo "🎯 Try: os"