#!/bin/bash

# Server OS Main Installation Script
# Installs only the core server-os REPL

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo "🖥️  Server OS Installation"
echo "=========================="
echo ""
echo "Version: $(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)"
echo ""

# Check for Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "📦 Building and installing Server OS REPL..."
cargo build --release
cargo install --path .

echo ""
echo "✅ Server OS installed successfully!"
echo ""
echo "🚀 Usage: os"
echo ""
echo "Available commands in REPL:"
echo "  :help     - Show all available commands"
echo "  :finder   - Launch file manager (requires yazi)"
echo "  :system   - Launch system monitor (requires btm)"
echo "  :network  - Launch network monitor (requires bandwhich)"
echo "  :trace    - Network diagnostics (requires trippy)"
echo "  :update   - Update Server OS to latest version"
echo "  :exit     - Exit the REPL"
echo ""
echo "📋 Shell commands can be run directly without prefix"
echo ""
echo "📦 Optional installations:"
echo "  ./install-tools.sh    - Install file managers and monitors"
echo "  ./install-security.sh - Install security tools"
echo ""
echo "🎯 Try: os