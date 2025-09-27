#!/bin/bash

# Server OS Security Tools Installation Script
# These are optional security tools that can be installed separately

set -e

echo "🛡️  Server OS Security Tools Installer"
echo "======================================="
echo ""

# Function to install a security tool
install_security() {
    local name=$1
    local path=$2
    local description=$3

    echo -n "Install $description? (y/n): "
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        echo "🔒 Installing $name..."
        cargo install --path "$path"
        echo "✅ $name installed!"
    else
        echo "⏭️  Skipping $name"
    fi
    echo ""
}

echo "🔥 Firewall Tools:"
install_security "dfw" "security/dfw-main" "DFW Docker Firewall"

echo "🌐 Web Security:"
install_security "aegis" "security/aegis-main" "Aegis Web Application Firewall"

echo ""
echo "✅ Security tools installation complete!"
echo ""
echo "Installed security commands:"
echo "  dfw   - Docker firewall (needs config at /etc/dfw/dfw.toml)"
echo "  aegis - Web application firewall"
echo ""
echo "⚠️  Note: Security tools require additional configuration."
echo "  Please refer to their documentation for setup instructions."