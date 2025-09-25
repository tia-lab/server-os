#!/bin/bash

echo "🚀 Installing Server OS Tools..."

# Core TUI Tools
echo "📁 Installing Yazi file manager..."
cargo install yazi-fm yazi-cli

echo "📊 Installing bottom system monitor..."
cargo install bottom

echo "🌐 Installing bandwhich network monitor..."
cargo install bandwhich

# Optional: Install additional tools
echo "🔍 Installing ripgrep for searching..."
cargo install ripgrep

echo "⚡ Installing fd for finding files..."
cargo install fd-find

echo "📖 Installing bat for viewing files..."
cargo install bat

# Install our OS wrapper
echo "🖥️ Installing OS wrapper..."
cargo install --path .

echo "✅ Installation complete!"
echo ""
echo "Available commands:"
echo "  yazi      - File manager"
echo "  btm       - System monitor (bottom)"
echo "  bandwhich - Network monitor"
echo "  rg        - Search files (ripgrep)"
echo "  fd        - Find files"
echo "  bat       - View files with syntax highlighting"
echo "  os        - Server OS dashboard (press f/s/n for tools)"
echo ""
echo "🎯 Try: os"
