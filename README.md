# 🛡️ Server OS - Security-Hardened Server REPL

A lightweight, security-focused REPL for server management with integrated tools and monitoring capabilities.

## 🚀 Overview

Server OS provides:
- **Interactive REPL** with command history and tab completion
- **Security integration** with system monitoring capabilities
- **Tool integration** for file management, system monitoring, and network analysis
- **Self-updating** mechanism to stay current with latest features
- **Shell pass-through** for direct command execution

## 🎯 Quick Start

```bash
# Clone the repository
git clone https://github.com/tia-lab/server-os.git
cd server-os

# Install the core REPL
./install.sh

# Run Server OS
os
```

## 📦 Installation Options

### Core Installation
```bash
./install.sh  # Installs only the server-os REPL
```

### Optional Tools
```bash
./install-tools.sh     # Interactive installer for file managers and monitors
./install-security.sh  # Interactive installer for security tools
```

## 🎛️ Available Commands

Once in the REPL (`os`), use these commands:

| Command | Description | Requires Tool |
|---------|-------------|---------------|
| `:help` | Show all available commands | - |
| `:status` | Display system security status | - |
| `:update` | Update Server OS to latest version | - |
| `:exit` | Exit the REPL | - |
| `:finder` | Launch Yazi file manager | yazi |
| `:system` | Launch bottom system monitor | btm |
| `:network` | Launch bandwhich network monitor | bandwhich |
| `:trace` | Network diagnostics with trippy | trippy |
| `:firewall` | Docker firewall configuration | dfw |
| `:waf` | Web application firewall | aegis |

**Note**: Shell commands can be run directly without any prefix.

## 🔄 Self-Updating

Server OS includes a built-in update mechanism:

```bash
# From within the REPL
os> :update
```

This will:
1. Pull the latest version from GitHub
2. Rebuild the application
3. Install the updated version
4. Prompt you to restart

## 🛠️ Tool Stack

### Core TUI Tools (Optional)
| Tool | Purpose | Install Via |
|------|---------|------------|
| [yazi](https://github.com/sxyazi/yazi) | Terminal file manager | `install-tools.sh` |
| [xplr](https://github.com/sayanarijit/xplr) | File explorer | `install-tools.sh` |
| [bottom](https://github.com/ClementTsang/bottom) | System monitor | `install-tools.sh` |
| [bandwhich](https://github.com/imsnif/bandwhich) | Network monitor | `install-tools.sh` |
| [trippy](https://github.com/fujiapple852/trippy) | Network diagnostics | `install-tools.sh` |

### Security Tools (Optional)
| Tool | Purpose | Install Via |
|------|---------|------------|
| [DFW](https://github.com/pitkley/dfw) | Docker firewall | `install-security.sh` |
| [Aegis](https://github.com/utibeabasi6/aegis) | Web application firewall | `install-security.sh` |

## 📁 Project Structure

```
server-os/
├── src/
│   └── main.rs           # Core REPL implementation
├── tools/                # Optional tool sources
│   ├── yazi-main/        # File manager
│   ├── xplr-main/        # File explorer
│   ├── bottom-main/      # System monitor
│   ├── bandwhich-main/   # Network monitor
│   └── trippy-master/    # Network diagnostics
├── security/             # Security tool sources
│   ├── dfw-main/         # Docker firewall
│   └── aegis-main/       # Web application firewall
├── install.sh            # Main installer (REPL only)
├── install-tools.sh      # Optional tools installer
├── install-security.sh   # Security tools installer
├── Cargo.toml            # Project configuration
└── README.md             # This file
```

## 🛡️ Security Features

Server OS integrates several security-focused Rust crates:

- **sysinfo**: System monitoring and process management
- **notify**: File system event monitoring
- **pnet**: Network packet analysis
- **ring**: Cryptographic operations

## 💻 Requirements

- **Rust** 1.70+ (install via [rustup](https://rustup.rs/))
- **Git** (for update functionality)
- **OS**: Linux/macOS (Windows support via WSL)

## 🚀 Usage Examples

```bash
# Launch Server OS
os

# Check system status
os> :status

# Run shell commands directly
os> ls -la
os> docker ps
os> systemctl status nginx

# Launch integrated tools (if installed)
os> :finder   # File manager
os> :system   # System monitor
os> :network  # Network monitor

# Update to latest version
os> :update

# Exit
os> :exit
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues on the [GitHub repository](https://github.com/tia-lab/server-os).

## 📄 License

This project is licensed under the MIT License.

---

**Server OS** - *Lightweight, Secure, Efficient Server Management*