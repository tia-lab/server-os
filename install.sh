#!/bin/bash

# 🛡️ server-os Installation Script
# Security-Hardened Server Operating System
# Complete Rust TUI Stack + Security Tools

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
INSTALL_DIR="/opt/server-os"
BIN_DIR="/usr/local/bin"
SERVICE_DIR="/etc/systemd/system"

# Print colored output
print_status() {
    echo -e "${CYAN}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        print_error "This script must be run as root"
        exit 1
    fi
}

# Detect OS
detect_os() {
    if [[ -f /etc/os-release ]]; then
        . /etc/os-release
        OS=$NAME
        VER=$VERSION_ID
    else
        print_error "Cannot detect OS"
        exit 1
    fi

    print_status "Detected OS: $OS $VER"
}

# Install dependencies
install_dependencies() {
    print_status "Installing system dependencies..."

    case "$OS" in
        "Ubuntu"*)
            apt update
            apt install -y \
                build-essential \
                curl wget git \
                pkg-config libssl-dev \
                libpcap-dev \
                iptables \
                unzip
            ;;
        "CentOS"*|"Red Hat"*|"Fedora"*)
            if command -v dnf &> /dev/null; then
                dnf update -y
                dnf install -y \
                    gcc gcc-c++ make \
                    curl wget git \
                    pkg-config openssl-devel \
                    libpcap-devel \
                    iptables \
                    unzip
            else
                yum update -y
                yum install -y \
                    gcc gcc-c++ make \
                    curl wget git \
                    pkg-config openssl-devel \
                    libpcap-devel \
                    iptables \
                    unzip
            fi
            ;;
        *)
            print_error "Unsupported OS: $OS"
            exit 1
            ;;
    esac

    print_success "Dependencies installed"
}

# Install Rust
install_rust() {
    if command -v cargo &> /dev/null; then
        print_status "Rust already installed: $(rustc --version)"
        return
    fi

    print_status "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    print_success "Rust installed: $(rustc --version)"
}

# Create directories
create_directories() {
    print_status "Creating directories..."
    mkdir -p "$INSTALL_DIR"/{bin,configs,logs,tools,security}
    mkdir -p "$BIN_DIR"
    print_success "Directories created"
}

# Build and install tools
build_tools() {
    print_status "Building server-os tools..."

    # Create temporary build directory
    BUILD_DIR=$(mktemp -d)
    cd "$BUILD_DIR"

    # Download server-os
    git clone https://github.com/yourusername/server-os.git
    cd server-os

    # Build OS launcher
    print_status "Building OS dashboard launcher..."
    cargo build --release
    cp target/release/os "$BIN_DIR/"
    chmod +x "$BIN_DIR/os"

    # Build core tools
    cd src/tools

    # Build trippy (network diagnostics)
    if [[ -d "trippy-master" ]]; then
        print_status "Building trippy..."
        cd trippy-master
        cargo build --release
        cp target/release/trip "$BIN_DIR/trace"
        chmod +x "$BIN_DIR/trace"
        cd ..
    fi

    # Build bandwhich (network monitor)
    if [[ -d "bandwhich-main" ]]; then
        print_status "Building bandwhich..."
        cd bandwhich-main
        cargo build --release
        cp target/release/bandwhich "$BIN_DIR/network"
        chmod +x "$BIN_DIR/network"
        cd ..
    fi

    # Build security tools
    cd ../security

    # Build Heimdall (intrusion detection)
    if [[ -d "heimdall-master" ]]; then
        print_status "Building Heimdall intrusion detection..."
        cd heimdall-master
        cargo build --release
        cp target/release/heimdall "$BIN_DIR/guard"
        chmod +x "$BIN_DIR/guard"
        cd ..
    fi

    # Build DFW (Docker firewall)
    if [[ -d "dfw-main" ]]; then
        print_status "Building DFW firewall..."
        cd dfw-main
        cargo build --release
        cp target/release/dfw "$BIN_DIR/firewall"
        chmod +x "$BIN_DIR/firewall"
        cd ..
    fi

    # Build Aegis (WAF)
    if [[ -d "aegis-main" ]]; then
        print_status "Building Aegis WAF..."
        cd aegis-main
        cargo build --release
        cp target/release/aegis "$BIN_DIR/waf"
        chmod +x "$BIN_DIR/waf"
        cd ..
    fi

    # Install other pre-built tools
    print_status "Installing additional tools..."

    # xplr (file manager)
    XPLR_VERSION=$(curl -s https://api.github.com/repos/sayanarijit/xplr/releases/latest | grep -Po '"tag_name": "\K.*?(?=")')
    wget -O /tmp/xplr.tar.gz "https://github.com/sayanarijit/xplr/releases/download/${XPLR_VERSION}/xplr-linux.tar.gz"
    tar -xzf /tmp/xplr.tar.gz -C /tmp/
    cp /tmp/xplr "$BIN_DIR/finder"
    chmod +x "$BIN_DIR/finder"

    # television (fuzzy finder)
    TV_VERSION=$(curl -s https://api.github.com/repos/alexpasmantier/television/releases/latest | grep -Po '"tag_name": "\K.*?(?=")')
    wget -O /tmp/television.tar.gz "https://github.com/alexpasmantier/television/releases/download/${TV_VERSION}/television-${TV_VERSION}-x86_64-unknown-linux-gnu.tar.gz"
    tar -xzf /tmp/television.tar.gz -C /tmp/
    cp /tmp/television "$BIN_DIR/search"
    chmod +x "$BIN_DIR/search"

    # bottom (system monitor)
    BOTTOM_VERSION=$(curl -s https://api.github.com/repos/ClementTsang/bottom/releases/latest | grep -Po '"tag_name": "\K.*?(?=")')
    wget -O /tmp/bottom.tar.gz "https://github.com/ClementTsang/bottom/releases/download/${BOTTOM_VERSION}/bottom_x86_64-unknown-linux-gnu.tar.gz"
    tar -xzf /tmp/bottom.tar.gz -C /tmp/
    cp /tmp/btm "$BIN_DIR/system"
    chmod +x "$BIN_DIR/system"

    # Clean up
    rm -rf "$BUILD_DIR"
    rm -f /tmp/xplr.tar.gz /tmp/television.tar.gz /tmp/bottom.tar.gz

    print_success "Tools built and installed"
}

# Install security tools
install_security_tools() {
    print_status "Configuring server-os security tools..."

    # Create server-os configuration directories
    mkdir -p /etc/server-os/{security,configs}
    mkdir -p /opt/server-os/{tools,security,backups}
    mkdir -p /var/log/server-os/security

    # Copy unified configuration
    cp server-os.toml /etc/server-os/server-os.toml
    chmod 644 /etc/server-os/server-os.toml

    # Set up basic iptables rules (will be managed by DFW)
    print_status "Setting up basic firewall rules..."

    # Enable IP forwarding for Docker
    echo 'net.ipv4.ip_forward=1' >> /etc/sysctl.conf
    sysctl -p

    print_success "server-os security tools configured"
}

# Create service files
create_services() {
    print_status "Creating systemd services..."

    # Create server-os monitoring service
    cat > "$SERVICE_DIR/server-os-monitor.service" << EOF
[Unit]
Description=server-os System Monitor
After=network.target

[Service]
Type=simple
User=root
ExecStart=$BIN_DIR/system
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

    systemctl daemon-reload
    print_success "Services created"
}

# Main installation function
main() {
    print_status "🛡️ Starting server-os installation..."

    check_root
    detect_os
    install_dependencies
    install_rust
    create_directories
    build_tools
    install_security_tools
    create_services

    print_success "🎉 server-os installation completed!"
    print_status ""
    print_status "🚀 Quick Start:"
    print_status "  Launch OS dashboard: ${CYAN}os${NC}"
    print_status ""
    print_status "🛠️ Available Commands:"
    print_status "  finder   - Interactive file manager (xplr)"
    print_status "  search   - Fuzzy finder (television)"
    print_status "  system   - System monitor (bottom)"
    print_status "  network  - Network monitor (bandwhich)"
    print_status "  trace    - Network diagnostics (trippy)"
    print_status ""
    print_status "🛡️ Security Tools:"
    print_status "  guard    - Intrusion detection (Heimdall)"
    print_status "  firewall - Docker firewall (DFW)"
    print_status "  waf      - Web application firewall (Aegis)"
    print_status ""
    print_status "📚 Documentation: https://github.com/yourusername/server-os"
}

# Run main function
main "$@"