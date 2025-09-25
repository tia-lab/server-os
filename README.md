# 🛡️ server-os

**Security-Hardened Server Operating System**
*Complete Rust TUI Stack + Security Tools*

## 🚀 Overview

server-os is a golden standard server environment that provides:
- **Complete TUI stack** for file management, system monitoring, and network analysis
- **Security-hardened tools** for intrusion detection and firewall management
- **Zero sync issues** - direct server operations, perfect VS Code Remote SSH compatibility
- **100% Rust** - consistent performance, memory safety, zero garbage collection

## 🎯 Quick Start

```bash
# Deploy to any server
curl -sSL https://raw.githubusercontent.com/yourusername/server-os/main/install.sh | bash

# SSH into server
ssh root@your-server

# Launch OS dashboard
os
```

## 🛠️ Tool Stack

### Core TUI Tools (6)
| Command | Tool | Purpose |
|---------|------|---------|
| `finder` | [xplr](https://github.com/sayanarijit/xplr) | Interactive file manager |
| `search` | [television](https://github.com/alexpasmantier/television) | Fuzzy finder |
| `disk` | [wiper](https://github.com/ikebastuz/wiper) | Disk analyzer |
| `system` | [bottom](https://github.com/ClementTsang/bottom) | System monitor |
| `network` | [bandwhich](https://github.com/imsnif/bandwhich) | Network monitor |
| `trace` | [trippy](https://github.com/fujiapple852/trippy) | Network diagnostics |

### Security Tools (3)
| Command | Tool | Purpose |
|---------|------|---------|
| `guard` | [Heimdall](https://github.com/acriba/heimdall) | Intrusion detection (fail2ban alternative) |
| `firewall` | [DFW](https://github.com/pitkley/dfw) | Docker firewall framework |
| `waf` | [Aegis](https://github.com/utibeabasi6/aegis) | Web application firewall |

## 🎛️ OS Dashboard

The `os` command launches a comprehensive ratatui-based dashboard:

```
┌─ Secure Server OS Dashboard ───────────────────────────┐
│  ╭─ Tools ─────╮    ╭─ Security Status ──╮            │
│  │ 📁 finder    │    │ 🔒 Firewall: Active │            │
│  │ 🔍 search    │    │ 🛡️  IDS: Monitoring  │            │
│  │ 💾 disk      │    │ 🚫 Blocked: 42 IPs   │            │
│  │ 📊 system    │    │ ⚠️  Alerts: 3 new    │            │
│  │ 🌐 network   │    ╰─────────────────────╯            │
│  │ 📍 trace     │                                      │
│  ╰─────────────╯    ╭─ Security Tools ────╮            │
│                     │ 🛡️  guard (IDS)      │            │
│  ╭─ System ────╮    │ 🔥 firewall (UFW)    │            │
│  │ CPU: ███░░   │    │ 🌐 waf (WAF)        │            │
│  │ RAM: ████░   │    │ 🔍 logs             │            │
│  ╰─────────────╯    ╰─────────────────────╯            │
└────────────────────────────────────────────────────────┘
```

## 📁 Project Structure

```
server-os/
├── launcher/           # Ratatui OS dashboard
│   ├── src/main.rs     # Dashboard implementation
│   └── Cargo.toml      # Dependencies
├── src/
│   ├── tools/         # TUI tool source code
│   │   ├── trippy-master/    # Network diagnostics
│   │   └── bandwhich-main/   # Network monitor
│   └── security/      # Security tool source code
│       ├── heimdall-master/  # Intrusion detection
│       ├── dfw-main/         # Docker firewall
│       └── aegis-main/       # Web application firewall
├── install.sh         # Main installer
├── Cargo.toml         # Workspace configuration
└── README.md          # This file
```

## 🛡️ Security Features

- **Intrusion Detection**: Heimdall monitors logs and blocks malicious IPs
- **Firewall Management**: DFW provides container-aware firewall rules  
- **Web Protection**: Aegis WAF filters malicious web requests
- **Network Monitoring**: Real-time bandwidth and connection tracking
- **Log Analysis**: Centralized security event monitoring

## 🚀 Performance

- **Zero sync issues** with VS Code Remote SSH
- **Sub-second startup** for all tools  
- **Memory efficient** - no garbage collection overhead
- **CPU optimized** - Rust's zero-cost abstractions
- **Network optimized** - async I/O throughout

---

**🛡️ server-os** - *Secure, Fast, Reliable Server Management*
