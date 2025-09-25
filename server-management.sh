#!/bin/bash

# Safe Server Management Script
# This runs ON YOUR MAC and sends commands to your server

SERVER_IP="65.21.198.30"
SERVER_USER="root"

echo "🛡️ Server Management Tool"
echo "========================"
echo ""
echo "⚠️  SECURITY FIRST:"
echo "1. Change your root password immediately!"
echo "2. Set up SSH keys instead of passwords"
echo "3. Never share credentials publicly"
echo ""

# Function to run safe commands on server
run_server_command() {
    local cmd="$1"
    echo "Running: $cmd"
    ssh "${SERVER_USER}@${SERVER_IP}" "$cmd"
}

# Menu
echo "Choose an action:"
echo "1. Check server status"
echo "2. Install server-os"
echo "3. Deploy application"
echo "4. View logs"
echo "5. SECURE SERVER (recommended!)"

read -p "Enter choice: " choice

case $choice in
    1)
        echo "📊 Checking server status..."
        run_server_command "uname -a && free -h && df -h && uptime"
        ;;
    2)
        echo "🚀 Installing server-os..."
        run_server_command "cd /opt && git clone https://github.com/yourusername/server-os && cd server-os && ./install.sh"
        ;;
    3)
        echo "📦 Deploying application..."
        rsync -avz --exclude 'target' --exclude '.git' . "${SERVER_USER}@${SERVER_IP}:/opt/server-os/"
        ;;
    4)
        echo "📜 Viewing logs..."
        run_server_command "journalctl -n 50"
        ;;
    5)
        echo "🔐 Securing server..."
        cat << 'SECURE_SCRIPT' | ssh "${SERVER_USER}@${SERVER_IP}" bash
# Change password
echo "Please change root password:"
passwd

# Create new user
read -p "Enter new username: " NEW_USER
adduser $NEW_USER
usermod -aG sudo $NEW_USER

# Setup firewall
ufw allow OpenSSH
ufw --force enable

# Secure SSH
sed -i 's/PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config
sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
systemctl restart sshd

echo "✅ Server secured! Use SSH keys from now on."
SECURE_SCRIPT
        ;;
esac