#!/bin/bash

# Server Assistant - Safe execution with logging
# This runs on YOUR SERVER with your supervision

LOG_FILE="/var/log/server-assistant.log"
SAFE_MODE=true

# Function to safely run commands with logging
safe_run() {
    local cmd="$1"
    local description="$2"

    echo "═══════════════════════════════════════"
    echo "📋 Task: $description"
    echo "💻 Command: $cmd"

    if [ "$SAFE_MODE" = true ]; then
        read -p "Execute this command? (y/n): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "⏭️  Skipped"
            return
        fi
    fi

    echo "🔄 Running..."
    echo "[$(date)] $cmd" >> "$LOG_FILE"

    eval "$cmd"

    if [ $? -eq 0 ]; then
        echo "✅ Success"
    else
        echo "❌ Failed"
    fi
    echo
}

# Main menu
show_menu() {
    echo "
╔══════════════════════════════════════╗
║     🤖 SERVER ASSISTANT WITH CLAUDE   ║
╚══════════════════════════════════════╝

Choose what you need help with:

1. 🔧 Initial Server Setup
2. 🦀 Install Rust & Development Tools
3. 🛡️  Install server-os
4. 🐳 Setup Docker
5. 🔒 Security Hardening
6. 📊 Performance Tuning (for 512GB RAM!)
7. 🌐 Configure Networking
8. 💾 Setup Backups
9. 📝 Custom Task (I'll help write it)
0. 🚪 Exit

"
    read -p "Select option: " choice

    case $choice in
        1) initial_setup ;;
        2) install_rust ;;
        3) install_server_os ;;
        4) setup_docker ;;
        5) security_hardening ;;
        6) performance_tuning ;;
        7) configure_networking ;;
        8) setup_backups ;;
        9) custom_task ;;
        0) exit 0 ;;
        *) echo "Invalid option" ;;
    esac
}

# 1. Initial Setup
initial_setup() {
    echo "🔧 Starting Initial Server Setup..."

    safe_run "apt update && apt upgrade -y" "Update system packages"
    safe_run "apt install -y build-essential git vim curl wget htop" "Install essential tools"
    safe_run "timedatectl set-timezone Europe/Helsinki" "Set Helsinki timezone"
    safe_run "hostnamectl set-hostname helsinki-beast" "Set hostname"

    show_menu
}

# 2. Install Rust
install_rust() {
    echo "🦀 Installing Rust Development Environment..."

    safe_run "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y" "Install Rust"
    safe_run "source $HOME/.cargo/env" "Setup Rust environment"
    safe_run "rustup default stable" "Set stable Rust"
    safe_run "cargo install cargo-watch cargo-edit" "Install cargo tools"

    show_menu
}

# 3. Install server-os
install_server_os() {
    echo "🛡️ Installing server-os..."

    safe_run "cd /opt && git clone https://github.com/yourusername/server-os" "Clone repository"
    safe_run "cd /opt/server-os && ./install.sh" "Run installation"
    safe_run "echo 'alias os=/opt/server-os/os' >> ~/.bashrc" "Create alias"

    show_menu
}

# 4. Setup Docker
setup_docker() {
    echo "🐳 Setting up Docker..."

    safe_run "apt install -y docker.io docker-compose" "Install Docker"
    safe_run "systemctl enable docker && systemctl start docker" "Enable Docker"
    safe_run "docker run hello-world" "Test Docker installation"

    show_menu
}

# 5. Security Hardening
security_hardening() {
    echo "🔒 Hardening Server Security..."

    safe_run "apt install -y ufw fail2ban" "Install security tools"
    safe_run "ufw allow 22/tcp && ufw allow 80/tcp && ufw allow 443/tcp" "Configure firewall rules"
    safe_run "ufw --force enable" "Enable firewall"
    safe_run "systemctl enable fail2ban && systemctl start fail2ban" "Enable fail2ban"

    echo "⚠️  Remember to:"
    echo "  - Set up SSH keys"
    echo "  - Disable root login"
    echo "  - Change default ports"

    show_menu
}

# 6. Performance Tuning for 512GB RAM
performance_tuning() {
    echo "📊 Optimizing for 512GB RAM & Xeon W-2295..."

    safe_run "echo 'vm.swappiness=10' >> /etc/sysctl.conf" "Reduce swap usage"
    safe_run "echo 'vm.max_map_count=262144' >> /etc/sysctl.conf" "Increase memory maps"
    safe_run "echo 'net.core.somaxconn=65535' >> /etc/sysctl.conf" "Increase connection queue"
    safe_run "sysctl -p" "Apply settings"

    show_menu
}

# 7. Configure Networking
configure_networking() {
    echo "🌐 Configuring Network..."

    safe_run "ip addr show" "Show network interfaces"
    safe_run "systemctl status networking" "Check network status"
    safe_run "netstat -tulpn" "Show listening ports"

    show_menu
}

# 8. Setup Backups
setup_backups() {
    echo "💾 Setting up Backup System..."

    safe_run "apt install -y rsync borgbackup" "Install backup tools"
    safe_run "mkdir -p /backup/{daily,weekly,monthly}" "Create backup directories"

    cat > /tmp/backup.sh << 'EOF'
#!/bin/bash
# Daily backup script
rsync -av --exclude='/proc' --exclude='/sys' --exclude='/dev' / /backup/daily/
EOF

    safe_run "mv /tmp/backup.sh /usr/local/bin/ && chmod +x /usr/local/bin/backup.sh" "Install backup script"
    safe_run "echo '0 2 * * * /usr/local/bin/backup.sh' | crontab -" "Schedule daily backup"

    show_menu
}

# 9. Custom Task
custom_task() {
    echo "📝 Custom Task Assistant"
    echo "Describe what you want to do, and I'll help create the commands."
    echo "Example: 'Install PostgreSQL and create a database'"

    read -p "What do you need help with? " task

    echo "
Based on your request: '$task'
I would suggest these commands:

1. First, we should...
   [Claude will provide specific commands based on the task]

2. Then...
   [More steps as needed]

"

    echo "💡 Tip: Share this task with Claude for specific commands!"

    show_menu
}

# Start the assistant
echo "Starting Server Assistant..."
echo "All commands will be logged to: $LOG_FILE"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "⚠️  Please run as root: sudo $0"
    exit 1
fi

show_menu