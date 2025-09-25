#!/bin/bash

echo "🗑️ Uninstalling Server OS Tools..."

# Confirm before uninstalling
read -p "Are you sure you want to uninstall all Server OS tools? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Uninstall cancelled."
    exit 1
fi

# Function to safely remove a binary if it exists
remove_binary() {
    local binary_name="$1"
    local binary_path="$HOME/.cargo/bin/$binary_name"

    if [ -f "$binary_path" ]; then
        echo "🗑️ Removing $binary_name..."
        rm -f "$binary_path"
        if [ $? -eq 0 ]; then
            echo "   ✅ $binary_name removed successfully"
        else
            echo "   ❌ Failed to remove $binary_name"
        fi
    else
        echo "   ⚠️ $binary_name not found, skipping..."
    fi
}

# Remove core TUI tools
echo "📁 Removing Yazi file manager..."
remove_binary "yazi"
remove_binary "yazi-cli"

echo "📊 Removing bottom system monitor..."
remove_binary "btm"

echo "🌐 Removing bandwhich network monitor..."
remove_binary "bandwhich"

echo "🔍 Removing xplr file explorer..."
remove_binary "xplr"

echo "🌐 Removing trippy network diagnostics..."
remove_binary "trip"

# Remove security tools
echo "🔥 Removing DFW Docker Firewall..."
remove_binary "dfw"

echo "🌐 Removing Aegis WAF..."
remove_binary "aegis"

# Remove external tools (optional - ask user)
echo ""
read -p "Remove external tools (ripgrep, fd-find, bat)? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🔍 Removing ripgrep..."
    remove_binary "rg"

    echo "⚡ Removing fd-find..."
    remove_binary "fd"

    echo "📖 Removing bat..."
    remove_binary "bat"
fi

# Remove our OS wrapper (includes integrated security crates)
echo "🖥️ Removing OS wrapper with security integrations..."
echo "   📦 Also removes: sysinfo, notify, pnet, ring dependencies"
remove_binary "os"

# Remove history file
echo "🗂️ Removing history file..."
if [ -f ".server-os-history" ]; then
    rm -f ".server-os-history"
    echo "   ✅ History file removed"
fi

# Remove any cached build artifacts in target directory
echo "🧹 Cleaning build artifacts..."
if [ -d "target" ]; then
    rm -rf "target"
    echo "   ✅ Build artifacts cleaned"
fi

echo ""
echo "✅ Uninstall complete!"
echo ""
echo "Note: Source code and configuration files have been preserved."
echo "If you want to completely remove the project, delete the entire directory."
echo ""
echo "To reinstall, run: ./install.sh"