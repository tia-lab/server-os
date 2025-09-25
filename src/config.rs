use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerOsConfig {
    pub server: ServerConfig,
    pub dashboard: DashboardConfig,
    pub tools: ToolsConfig,
    pub security: SecurityConfig,
    pub monitoring: MonitoringConfig,
    pub logging: LoggingConfig,
    pub network: NetworkConfig,
    pub storage: StorageConfig,
    pub performance: PerformanceConfig,
    pub integrations: IntegrationsConfig,
    pub updates: UpdatesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub version: String,
    pub hostname: String,
    pub timezone: String,
    pub log_level: String,
    pub log_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub title: String,
    pub refresh_interval: u64,
    pub theme: String,
    pub show_system_info: bool,
    pub show_security_status: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub data_dir: String,
    pub finder: ToolConfig,
    pub search: SearchConfig,
    pub disk: DiskConfig,
    pub system: SystemConfig,
    pub network: NetworkToolConfig,
    pub trace: TraceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub enabled: bool,
    pub command: String,
    pub config_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub enabled: bool,
    pub command: String,
    pub config_file: Option<String>,
    pub search_paths: Vec<String>,
    pub max_results: usize,
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskConfig {
    pub enabled: bool,
    pub command: String,
    pub scan_paths: Vec<String>,
    pub size_threshold: String,
    pub exclude_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub enabled: bool,
    pub command: String,
    pub config_file: Option<String>,
    pub update_rate: u64,
    pub show_process_tree: bool,
    pub show_network: bool,
    pub show_disks: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkToolConfig {
    pub enabled: bool,
    pub command: String,
    pub interface: String,
    pub show_dns: bool,
    pub show_connections: bool,
    pub raw_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceConfig {
    pub enabled: bool,
    pub command: String,
    pub config_file: Option<String>,
    pub default_target: String,
    pub packet_size: u32,
    pub max_hops: u32,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enabled: bool,
    pub data_dir: String,
    pub log_dir: String,
    pub alert_email: String,
    pub guard: GuardConfig,
    pub firewall: FirewallConfig,
    pub waf: WafConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardConfig {
    pub enabled: bool,
    pub command: String,
    pub config_file: Option<String>,
    pub log_files: Vec<String>,
    pub ban_time: u64,
    pub max_attempts: u32,
    pub whitelist_ips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallConfig {
    pub enabled: bool,
    pub command: String,
    pub config_file: Option<String>,
    pub backend_driver: String,
    pub default_policy: String,
    pub docker_integration: bool,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WafConfig {
    pub enabled: bool,
    pub command: String,
    pub config_file: Option<String>,
    pub listen_port: u16,
    pub backend_ports: Vec<u16>,
    pub rate_limit: u32,
    pub block_duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub cpu_threshold: f64,
    pub memory_threshold: f64,
    pub disk_threshold: f64,
    pub network_threshold: f64,
    pub alerts: AlertsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertsConfig {
    pub enabled: bool,
    pub email_notifications: bool,
    pub log_alerts: bool,
    pub webhook_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub rotation: String,
    pub max_size: String,
    pub max_files: u32,
    pub targets: LoggingTargets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingTargets {
    pub console: bool,
    pub file: bool,
    pub syslog: bool,
    pub journal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub interface: String,
    pub ipv6_enabled: bool,
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub data_retention_days: u32,
    pub log_retention_days: u32,
    pub backup_enabled: bool,
    pub backup_interval: String,
    pub backup_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub max_cpu_cores: u32,
    pub max_memory_gb: u32,
    pub io_priority: String,
    pub process_priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationsConfig {
    pub docker_enabled: bool,
    pub systemd_enabled: bool,
    pub prometheus_enabled: bool,
    pub prometheus_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatesConfig {
    pub auto_update: bool,
    pub check_interval: String,
    pub security_updates_only: bool,
    pub reboot_required: bool,
    pub maintenance_window: String,
}

impl ServerOsConfig {
    /// Load configuration from TOML file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;

        let config: ServerOsConfig = toml::from_str(&content)
            .with_context(|| "Failed to parse TOML configuration")?;

        Ok(config)
    }

    /// Load configuration with fallback to default locations
    pub fn load() -> Result<Self> {
        let config_paths = [
            "server-os.toml",
            "/etc/server-os/server-os.toml",
            "/opt/server-os/server-os.toml",
            "~/.config/server-os/server-os.toml",
        ];

        for path in &config_paths {
            if Path::new(path).exists() {
                return Self::load_from_file(path);
            }
        }

        // If no config file found, return default configuration
        Ok(Self::default())
    }

    /// Save configuration to TOML file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .with_context(|| "Failed to serialize configuration to TOML")?;

        fs::write(path.as_ref(), content)
            .with_context(|| format!("Failed to write config file: {:?}", path.as_ref()))?;

        Ok(())
    }

    /// Get tool configuration by name
    pub fn get_tool_config(&self, tool_name: &str) -> Option<ToolCommand> {
        match tool_name {
            "finder" => Some(ToolCommand {
                name: "finder".to_string(),
                command: self.tools.finder.command.clone(),
                enabled: self.tools.finder.enabled,
                config_file: self.tools.finder.config_file.clone(),
                extra_args: vec![],
            }),
            "search" => Some(ToolCommand {
                name: "search".to_string(),
                command: self.tools.search.command.clone(),
                enabled: self.tools.search.enabled,
                config_file: self.tools.search.config_file.clone(),
                extra_args: self.tools.search.search_paths.iter()
                    .flat_map(|p| vec!["--path".to_string(), p.clone()])
                    .collect(),
            }),
            "disk" => Some(ToolCommand {
                name: "disk".to_string(),
                command: self.tools.disk.command.clone(),
                enabled: self.tools.disk.enabled,
                config_file: None,
                extra_args: self.tools.disk.scan_paths.clone(),
            }),
            "system" => Some(ToolCommand {
                name: "system".to_string(),
                command: self.tools.system.command.clone(),
                enabled: self.tools.system.enabled,
                config_file: self.tools.system.config_file.clone(),
                extra_args: vec![],
            }),
            "network" => Some(ToolCommand {
                name: "network".to_string(),
                command: self.tools.network.command.clone(),
                enabled: self.tools.network.enabled,
                config_file: None,
                extra_args: if self.tools.network.interface != "all" {
                    vec!["--interface".to_string(), self.tools.network.interface.clone()]
                } else {
                    vec![]
                },
            }),
            "trace" => Some(ToolCommand {
                name: "trace".to_string(),
                command: self.tools.trace.command.clone(),
                enabled: self.tools.trace.enabled,
                config_file: self.tools.trace.config_file.clone(),
                extra_args: vec![self.tools.trace.default_target.clone()],
            }),
            "guard" => Some(ToolCommand {
                name: "guard".to_string(),
                command: self.security.guard.command.clone(),
                enabled: self.security.guard.enabled,
                config_file: self.security.guard.config_file.clone(),
                extra_args: vec![],
            }),
            "firewall" => Some(ToolCommand {
                name: "firewall".to_string(),
                command: self.security.firewall.command.clone(),
                enabled: self.security.firewall.enabled,
                config_file: self.security.firewall.config_file.clone(),
                extra_args: vec![],
            }),
            "waf" => Some(ToolCommand {
                name: "waf".to_string(),
                command: self.security.waf.command.clone(),
                enabled: self.security.waf.enabled,
                config_file: self.security.waf.config_file.clone(),
                extra_args: vec![],
            }),
            _ => None,
        }
    }

    /// Generate individual tool configuration files
    pub fn generate_tool_configs(&self) -> Result<HashMap<String, String>> {
        let mut configs = HashMap::new();

        // Generate xplr config
        if let Some(ref config_file) = self.tools.finder.config_file {
            let xplr_config = self.generate_xplr_config()?;
            configs.insert(config_file.clone(), xplr_config);
        }

        // Generate bottom config
        if let Some(ref config_file) = self.tools.system.config_file {
            let bottom_config = self.generate_bottom_config()?;
            configs.insert(config_file.clone(), bottom_config);
        }

        // Generate trippy config
        if let Some(ref config_file) = self.tools.trace.config_file {
            let trippy_config = self.generate_trippy_config()?;
            configs.insert(config_file.clone(), trippy_config);
        }

        // Generate security tool configs
        if let Some(ref config_file) = self.security.guard.config_file {
            let guard_config = self.generate_guard_config()?;
            configs.insert(config_file.clone(), guard_config);
        }

        if let Some(ref config_file) = self.security.firewall.config_file {
            let firewall_config = self.generate_firewall_config()?;
            configs.insert(config_file.clone(), firewall_config);
        }

        if let Some(ref config_file) = self.security.waf.config_file {
            let waf_config = self.generate_waf_config()?;
            configs.insert(config_file.clone(), waf_config);
        }

        Ok(configs)
    }

    fn generate_xplr_config(&self) -> Result<String> {
        Ok(format!(r#"---
version: v0.21.7
general:
  startup:
    path: "{}"
  default_ui:
    prefix: ""
    suffix: ""
    style:
      fg: "Reset"
      bg: "Reset"
      add_modifier: []
      sub_modifier: []
"#, self.tools.finder.config_file.as_ref().unwrap_or(&"/home".to_string())))
    }

    fn generate_bottom_config(&self) -> Result<String> {
        Ok(format!(r#"[flags]
dot_marker = false
rate = {}
left_legend = false
current_usage = false
unnormalized_cpu = false
group_processes = true
case_sensitive = false
whole_word = false
regex = false
basic = false
default_time_value = "60s"
time_delta = 60000
hide_time = false
temperature_type = "celsius"
default_widget_type = "proc"
default_widget_count = 1
expanded_on_startup = true
use_old_network_legend = false
hide_table_gap = false
show_table_scroll_position = false
process_command = false
disable_click = false
no_write = false
show_table_scroll_position = false
"#, self.tools.system.update_rate))
    }

    fn generate_trippy_config(&self) -> Result<String> {
        Ok(format!(r#"# trippy configuration
mode = "tui"
protocol = "icmp"
packet-size = {}
max-hops = {}
first-hop = 1
grace-duration = "100ms"
max-inflight = 24
initial-sequence = 33000
tos = 0
read-timeout = "10s"
dns-timeout = "5s"
dns-lookup-method = "system"
"#, self.tools.trace.packet_size, self.tools.trace.max_hops))
    }

    fn generate_guard_config(&self) -> Result<String> {
        Ok(format!(r#"# Heimdall intrusion detection configuration
ban_time = {}
max_attempts = {}
log_files = {:?}
whitelist_ips = {:?}
"#, self.security.guard.ban_time, self.security.guard.max_attempts,
     self.security.guard.log_files, self.security.guard.whitelist_ips))
    }

    fn generate_firewall_config(&self) -> Result<String> {
        Ok(format!(r#"# DFW firewall configuration
backend_driver = "{}"
default_policy = "{}"
docker_integration = {}
log_level = "{}"
"#, self.security.firewall.backend_driver, self.security.firewall.default_policy,
     self.security.firewall.docker_integration, self.security.firewall.log_level))
    }

    fn generate_waf_config(&self) -> Result<String> {
        Ok(format!(r#"# Aegis WAF configuration
listen_port = {}
backend_ports = {:?}
rate_limit = {}
block_duration = {}
"#, self.security.waf.listen_port, self.security.waf.backend_ports,
     self.security.waf.rate_limit, self.security.waf.block_duration))
    }
}

impl Default for ServerOsConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                name: "server-os".to_string(),
                version: "0.1.0".to_string(),
                hostname: "secure-server".to_string(),
                timezone: "UTC".to_string(),
                log_level: "info".to_string(),
                log_file: "/var/log/server-os/server-os.log".to_string(),
            },
            dashboard: DashboardConfig {
                title: "🛡️ server-os Dashboard".to_string(),
                refresh_interval: 1000,
                theme: "dark".to_string(),
                show_system_info: true,
                show_security_status: true,
            },
            tools: ToolsConfig {
                data_dir: "/opt/server-os/tools".to_string(),
                finder: ToolConfig {
                    enabled: true,
                    command: "xplr".to_string(),
                    config_file: Some("/opt/server-os/configs/xplr.yml".to_string()),
                },
                search: SearchConfig {
                    enabled: true,
                    command: "television".to_string(),
                    config_file: Some("/opt/server-os/configs/television.toml".to_string()),
                    search_paths: vec!["/home".to_string(), "/opt".to_string()],
                    max_results: 100,
                    case_sensitive: false,
                },
                disk: DiskConfig {
                    enabled: true,
                    command: "wiper".to_string(),
                    scan_paths: vec!["/".to_string(), "/home".to_string()],
                    size_threshold: "1GB".to_string(),
                    exclude_paths: vec!["/proc".to_string(), "/sys".to_string()],
                },
                system: SystemConfig {
                    enabled: true,
                    command: "bottom".to_string(),
                    config_file: Some("/opt/server-os/configs/bottom.toml".to_string()),
                    update_rate: 1000,
                    show_process_tree: true,
                    show_network: true,
                    show_disks: true,
                },
                network: NetworkToolConfig {
                    enabled: true,
                    command: "bandwhich".to_string(),
                    interface: "all".to_string(),
                    show_dns: true,
                    show_connections: true,
                    raw_mode: false,
                },
                trace: TraceConfig {
                    enabled: true,
                    command: "trippy".to_string(),
                    config_file: Some("/opt/server-os/configs/trippy.toml".to_string()),
                    default_target: "8.8.8.8".to_string(),
                    packet_size: 64,
                    max_hops: 30,
                    timeout: 5000,
                },
            },
            security: SecurityConfig {
                enabled: true,
                data_dir: "/opt/server-os/security".to_string(),
                log_dir: "/var/log/server-os/security".to_string(),
                alert_email: "admin@server.local".to_string(),
                guard: GuardConfig {
                    enabled: true,
                    command: "heimdall".to_string(),
                    config_file: Some("/opt/server-os/configs/heimdall.toml".to_string()),
                    log_files: vec!["/var/log/auth.log".to_string()],
                    ban_time: 3600,
                    max_attempts: 5,
                    whitelist_ips: vec!["127.0.0.1".to_string(), "::1".to_string()],
                },
                firewall: FirewallConfig {
                    enabled: true,
                    command: "dfw".to_string(),
                    config_file: Some("/opt/server-os/configs/dfw.toml".to_string()),
                    backend_driver: "iptables".to_string(),
                    default_policy: "drop".to_string(),
                    docker_integration: true,
                    log_level: "info".to_string(),
                },
                waf: WafConfig {
                    enabled: true,
                    command: "aegis".to_string(),
                    config_file: Some("/opt/server-os/configs/aegis.toml".to_string()),
                    listen_port: 8080,
                    backend_ports: vec![80, 443],
                    rate_limit: 100,
                    block_duration: 300,
                },
            },
            monitoring: MonitoringConfig {
                enabled: true,
                cpu_threshold: 80.0,
                memory_threshold: 85.0,
                disk_threshold: 90.0,
                network_threshold: 80.0,
                alerts: AlertsConfig {
                    enabled: true,
                    email_notifications: true,
                    log_alerts: true,
                    webhook_url: "".to_string(),
                },
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
                rotation: "daily".to_string(),
                max_size: "100MB".to_string(),
                max_files: 7,
                targets: LoggingTargets {
                    console: true,
                    file: true,
                    syslog: false,
                    journal: true,
                },
            },
            network: NetworkConfig {
                interface: "eth0".to_string(),
                ipv6_enabled: true,
                dns_servers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
            },
            storage: StorageConfig {
                data_retention_days: 30,
                log_retention_days: 90,
                backup_enabled: true,
                backup_interval: "daily".to_string(),
                backup_location: "/opt/server-os/backups".to_string(),
            },
            performance: PerformanceConfig {
                max_cpu_cores: 0,
                max_memory_gb: 0,
                io_priority: "normal".to_string(),
                process_priority: 0,
            },
            integrations: IntegrationsConfig {
                docker_enabled: true,
                systemd_enabled: true,
                prometheus_enabled: false,
                prometheus_port: 9090,
            },
            updates: UpdatesConfig {
                auto_update: false,
                check_interval: "daily".to_string(),
                security_updates_only: true,
                reboot_required: true,
                maintenance_window: "02:00-04:00".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToolCommand {
    pub name: String,
    pub command: String,
    pub enabled: bool,
    pub config_file: Option<String>,
    pub extra_args: Vec<String>,
}