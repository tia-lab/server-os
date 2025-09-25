#!/bin/bash

# 🚀 Server OS Deployment Script
# For production server deployment

set -e

echo "═══════════════════════════════════════════"
echo "   🛡️ SERVER-OS PRODUCTION DEPLOYMENT"
echo "═══════════════════════════════════════════"
echo ""
echo "Server: Intel Xeon W-2295 | 512GB RAM | 15TB SSD"
echo "Location: Helsinki, Finland"
echo ""

# Check if running as root (recommended for server)
if [[ $EUID -ne 0 ]]; then
   echo "⚠️  Not running as root. Some features may require sudo."
fi

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Clone repository if not exists
if [ ! -d "server-os" ]; then
    echo "📥 Cloning server-os repository..."
    git clone https://github.com/yourusername/server-os.git
    cd server-os
else
    echo "📂 Entering server-os directory..."
    cd server-os
    git pull
fi

# Optimize for Xeon W-2295 (Cascade Lake)
export RUSTFLAGS="-C target-cpu=cascadelake -C opt-level=3"

echo ""
echo "🔨 Building with Xeon optimizations..."
cargo build --release

echo ""
echo "📦 Installing server-os tools..."
./install.sh

# Create systemd service for auto-start (optional)
if [ -d "/etc/systemd/system" ]; then
    echo "🔧 Creating systemd service..."
    cat > /tmp/server-os.service << EOF
[Unit]
Description=Server OS Security Monitor
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME
ExecStart=$HOME/.cargo/bin/os --daemon
Restart=always

[Install]
WantedBy=multi-user.target
EOF

    echo "   To enable auto-start: sudo mv /tmp/server-os.service /etc/systemd/system/"
    echo "   Then: sudo systemctl enable server-os"
fi

# Configure for remote access
echo ""
echo "🌐 Configuring for SSH access..."
echo "alias os='$HOME/.cargo/bin/os'" >> ~/.bashrc
echo "alias server-status='os -c :status'" >> ~/.bashrc

# Create config directory
mkdir -p ~/.config/server-os

# Performance tuning for 512GB RAM
echo ""
echo "⚡ Applying performance optimizations..."
cat > ~/.config/server-os/performance.conf << EOF
# Performance settings for Intel Xeon W-2295
# 18 cores / 36 threads optimization

# Use all available cores for parallel operations
RAYON_NUM_THREADS=36

# Optimize for large memory (512GB)
MALLOC_ARENA_MAX=4

# Network optimization for 1Gbit
NET_BUFFER_SIZE=262144
EOF

echo ""
echo "✅ Deployment complete!"
echo ""
echo "═══════════════════════════════════════════"
echo "   🎯 Quick Start Commands:"
echo "═══════════════════════════════════════════"
echo ""
echo "  os           - Launch server-os REPL"
echo "  os :status   - Show system status"
echo "  os :finder   - File management"
echo "  os :system   - System monitor"
echo "  os :network  - Network monitor"
echo ""
echo "SSH from anywhere: ssh user@your-server-ip"
echo "Then run: os"
echo ""
echo "🛡️ Your server is ready for development!"