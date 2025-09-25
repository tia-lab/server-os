#!/bin/bash

echo "🚀 Installing Server OS Tools..."

# Core TUI Tools from our codebase
echo "📁 Installing xplr file manager..."
cargo install --path tools/xplr-main

echo "📁 Installing Yazi file manager..."
cargo install --path tools/yazi-main/yazi-fm
cargo install --path tools/yazi-main/yazi-cli

echo "📊 Installing bottom system monitor..."
cargo install --path tools/bottom-main

echo "🌐 Installing bandwhich network monitor..."
cargo install --path tools/bandwhich-main

echo "🌐 Installing trippy network diagnostics..."
cargo install --path tools/trippy-master/crates/trippy

# Security Tools
echo "🔥 Installing DFW Docker Firewall..."
cargo install --path security/dfw-main

echo "🌐 Installing Aegis WAF..."
cargo install --path security/aegis-main

# Optional: Install additional tools (external)
echo "🔍 Installing ripgrep for searching..."
cargo install ripgrep

echo "⚡ Installing fd for finding files..."
cargo install fd-find

echo "📖 Installing bat for viewing files..."
cargo install bat

# Install our OS wrapper with integrated security crates
echo "🖥️ Installing OS wrapper with security integrations..."
echo "   📦 Includes: sysinfo, notify, pnet, ring security crates"
cargo install --path .

echo "✅ Installation complete!"
echo ""
echo "Available commands:"
echo "  yazi      - File manager"
echo "  btm       - System monitor (bottom)"
echo "  bandwhich - Network monitor"
echo "  trip      - Network diagnostics (trippy)"
echo "  dfw       - Docker firewall (needs config)"
echo "  aegis     - Web application firewall"
echo "  rg        - Search files (ripgrep)"
echo "  fd        - Find files"
echo "  bat       - View files with syntax highlighting"
echo "  os        - Server OS dashboard with integrated security"
echo ""
echo "🛡️ Security Features:"
echo "  - System monitoring (sysinfo crate)"
echo "  - File integrity monitoring (notify crate)"
echo "  - Network packet analysis (pnet crate)"
echo "  - Cryptographic operations (ring crate)"
echo ""
echo "🎯 Try: os"
