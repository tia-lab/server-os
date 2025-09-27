#!/bin/bash

# Server OS Tools Installation Script
# These are optional tools that can be installed separately

set -e

echo "🚀 Server OS Optional Tools Installer"
echo "======================================"
echo ""

# Function to install a tool
install_tool() {
    local name=$1
    local path=$2
    local description=$3

    echo -n "Install $description? (y/n): "
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        echo "📦 Installing $name..."
        cargo install --path "$path"
        echo "✅ $name installed!"
    else
        echo "⏭️  Skipping $name"
    fi
    echo ""
}

# Core TUI Tools
echo "📁 File Managers:"
install_tool "xplr" "tools/xplr-main" "xplr file manager"
install_tool "yazi" "tools/yazi-main/yazi-fm" "Yazi file manager"

if [[ -d "tools/yazi-main/yazi-cli" ]]; then
    cargo install --path tools/yazi-main/yazi-cli 2>/dev/null || true
fi

echo "📊 System Monitoring:"
install_tool "bottom" "tools/bottom-main" "bottom system monitor"

echo "🌐 Network Tools:"
install_tool "bandwhich" "tools/bandwhich-main" "bandwhich network monitor"
install_tool "trippy" "tools/trippy-master/crates/trippy" "trippy network diagnostics"

echo "🔧 Additional Tools (from crates.io):"
echo -n "Install ripgrep for searching? (y/n): "
read -r response
if [[ "$response" =~ ^[Yy]$ ]]; then
    cargo install ripgrep
fi

echo -n "Install fd for finding files? (y/n): "
read -r response
if [[ "$response" =~ ^[Yy]$ ]]; then
    cargo install fd-find
fi

echo -n "Install bat for viewing files? (y/n): "
read -r response
if [[ "$response" =~ ^[Yy]$ ]]; then
    cargo install bat
fi

echo ""
echo "✅ Tools installation complete!"
echo ""
echo "Installed tools commands:"
echo "  yazi      - File manager"
echo "  xplr      - File manager"
echo "  btm       - System monitor"
echo "  bandwhich - Network monitor"
echo "  trip      - Network diagnostics"
echo "  rg        - Search files (if installed)"
echo "  fd        - Find files (if installed)"
echo "  bat       - View files (if installed)"