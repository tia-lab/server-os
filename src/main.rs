mod config;
mod tools;

use anyhow::Result;
use config::ServerOsConfig;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use sysinfo::System;
use tools::IntegratedTools;

// Tool definitions
#[derive(Debug, Clone)]
struct Tool {
    name: String,
    command: String,
    description: String,
    icon: String,
    category: ToolCategory,
}

#[derive(Debug, Clone, PartialEq)]
enum ToolCategory {
    Core,
    Security,
    System,
}

// Background system monitoring data
#[derive(Clone)]
struct SystemStats {
    cpu_usage: f64,
    memory_usage: f64,
    disk_usage: f64,
    uptime: u64,
    process_count: usize,
    load_avg: [f64; 3],
    network_rx: u64,
    network_tx: u64,
    temperatures: Vec<f32>,
}

impl Default for SystemStats {
    fn default() -> Self {
        SystemStats {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            uptime: 0,
            process_count: 0,
            load_avg: [0.0, 0.0, 0.0],
            network_rx: 0,
            network_tx: 0,
            temperatures: Vec::new(),
        }
    }
}

// Application state
struct App {
    tools: Vec<Tool>,
    current_tab: usize,
    selected_tool: usize,
    system: System,
    last_update: Instant,
    config: ServerOsConfig,
    last_error: Option<String>,
    system_stats: Arc<Mutex<SystemStats>>,
}

impl App {
    fn new() -> Result<Self> {
        // Load configuration
        let config = ServerOsConfig::load().unwrap_or_else(|e| {
            eprintln!("Warning: Failed to load config, using defaults: {}", e);
            ServerOsConfig::default()
        });

        let tools = vec![
            Tool {
                name: "finder".to_string(),
                command: config.tools.finder.command.clone(),
                description: "Interactive file manager".to_string(),
                icon: "📁".to_string(),
                category: ToolCategory::Core,
            },
            Tool {
                name: "system".to_string(),
                command: config.tools.system.command.clone(),
                description: "System monitor".to_string(),
                icon: "📊".to_string(),
                category: ToolCategory::System,
            },
            Tool {
                name: "network".to_string(),
                command: config.tools.network.command.clone(),
                description: "Network monitor".to_string(),
                icon: "🌐".to_string(),
                category: ToolCategory::System,
            },
            Tool {
                name: "trace".to_string(),
                command: config.tools.trace.command.clone(),
                description: "Network diagnostics".to_string(),
                icon: "📍".to_string(),
                category: ToolCategory::System,
            },
            Tool {
                name: "guard".to_string(),
                command: config.security.guard.command.clone(),
                description: "Intrusion detection".to_string(),
                icon: "🛡️".to_string(),
                category: ToolCategory::Security,
            },
            Tool {
                name: "firewall".to_string(),
                command: config.security.firewall.command.clone(),
                description: "Docker firewall".to_string(),
                icon: "🔥".to_string(),
                category: ToolCategory::Security,
            },
            Tool {
                name: "waf".to_string(),
                command: config.security.waf.command.clone(),
                description: "Web application firewall".to_string(),
                icon: "🌐".to_string(),
                category: ToolCategory::Security,
            },
        ];

        let mut system = System::new_all();
        system.refresh_all();

        let system_stats = Arc::new(Mutex::new(SystemStats::default()));

        Ok(Self {
            tools,
            current_tab: 0,
            selected_tool: 0,
            system,
            last_update: Instant::now(),
            config,
            last_error: None,
            system_stats,
        })
    }

    fn next_tool(&mut self) {
        self.selected_tool = (self.selected_tool + 1) % self.tools.len();
    }

    fn previous_tool(&mut self) {
        if self.selected_tool > 0 {
            self.selected_tool -= 1;
        } else {
            self.selected_tool = self.tools.len() - 1;
        }
    }

    fn next_tab(&mut self) {
        self.current_tab = (self.current_tab + 1) % 3;
    }

    fn previous_tab(&mut self) {
        if self.current_tab > 0 {
            self.current_tab -= 1;
        } else {
            self.current_tab = 2;
        }
    }

    fn launch_tool(&mut self) -> Result<()> {
        let tool = &self.tools[self.selected_tool];

        // Clear any previous error
        self.last_error = None;

        // Check if tool is available
        if !IntegratedTools::is_tool_available(&tool.name) {
            self.last_error = Some(format!("Tool '{}' is not available or disabled", tool.name));
            return Ok(());
        }

        // Get tool configuration
        if let Some(tool_cmd) = self.config.get_tool_config(&tool.name) {
            if !tool_cmd.enabled {
                self.last_error =
                    Some(format!("Tool '{}' is disabled in configuration", tool.name));
                return Ok(());
            }

            // Clear terminal
            disable_raw_mode()?;
            execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

            println!("🚀 Launching {}...", tool.name);

            // Prepare arguments from configuration
            let mut args = Vec::new();

            // Add config file if specified
            if let Some(ref config_file) = tool_cmd.config_file {
                args.push("--config".to_string());
                args.push(config_file.clone());
            }

            // Add extra arguments from configuration
            args.extend(tool_cmd.extra_args.clone());

            // Launch the integrated tool
            let result = IntegratedTools::launch_tool(&tool.name, &args);

            match result {
                Ok(_) => {
                    println!("✅ Tool '{}' completed successfully", tool.name);
                }
                Err(e) => {
                    self.last_error = Some(format!("Failed to launch {}: {}", tool.name, e));
                    println!("❌ Failed to launch {}: {}", tool.name, e);
                    println!("Press Enter to return to OS dashboard...");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                }
            }

            // Restore terminal
            enable_raw_mode()?;
            execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

            // Force system info refresh after tool exit
            self.system.refresh_all();
            self.last_update = Instant::now();
        } else {
            self.last_error = Some(format!("Tool '{}' not found in configuration", tool.name));
        }

        Ok(())
    }

    fn update_system_info(&mut self) {
        if self.last_update.elapsed() > Duration::from_secs(1) {
            self.system.refresh_all();
            self.last_update = Instant::now();
        }
    }

    fn launch_external_tool(&mut self, command: &str) -> Result<()> {
        use std::process::Command;

        // Clear any previous error
        self.last_error = None;

        // Clear terminal
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

        println!("🚀 Launching {}...", command);

        // Launch the external tool
        let status = Command::new(command).status()?;

        match status.success() {
            true => {
                println!("✅ Tool '{}' completed successfully", command);
            }
            false => {
                self.last_error = Some(format!("Tool '{}' failed with exit code: {:?}", command, status.code()));
                println!("❌ Tool '{}' failed", command);
                println!("Press Enter to return to OS dashboard...");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
        }

        // Restore terminal
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        // Force system info refresh after tool exit
        self.system.refresh_all();
        self.last_update = Instant::now();

        Ok(())
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new()?;

    // Main loop
    loop {
        app.update_system_info();

        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Esc => break,
                    KeyCode::Down | KeyCode::Char('j') => app.next_tool(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous_tool(),
                    KeyCode::Tab => app.next_tab(),
                    KeyCode::BackTab => app.previous_tab(),
                    KeyCode::Enter => {
                        if let Err(e) = app.launch_tool() {
                            app.last_error = Some(format!("Error launching tool: {}", e));
                        }
                    }
                    // Simple key mappings for global tools
                    KeyCode::Char('f') => {
                        if let Err(e) = app.launch_external_tool("yazi") {
                            app.last_error = Some(format!("Error launching Yazi: {}", e));
                        }
                    }
                    KeyCode::Char('s') => {
                        if let Err(e) = app.launch_external_tool("btm") {
                            app.last_error = Some(format!("Error launching bottom: {}", e));
                        }
                    }
                    KeyCode::Char('n') => {
                        if let Err(e) = app.launch_external_tool("bandwhich") {
                            app.last_error = Some(format!("Error launching bandwhich: {}", e));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(size);

    // Header
    let header = Paragraph::new(app.config.dashboard.title.clone())
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Main content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);

    // Tools panel
    render_tools_panel(f, app, main_chunks[0]);

    // Info panels
    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    render_system_info(f, app, info_chunks[0]);
    render_security_status(f, app, info_chunks[1]);

    // Footer
    let footer = Paragraph::new("f: Finder (yazi) | s: System (btm) | n: Network (bandwhich) | q/Esc: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn render_tools_panel(f: &mut Frame, app: &App, area: Rect) {
    let tools_items: Vec<ListItem> = app
        .tools
        .iter()
        .enumerate()
        .map(|(i, tool)| {
            let style = if i == app.selected_tool {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::raw(format!("{} ", tool.icon)),
                Span::styled(tool.name.clone(), style),
                Span::raw(" - "),
                Span::styled(tool.description.clone(), Style::default().fg(Color::Gray)),
            ]))
        })
        .collect();

    let tools_list = List::new(tools_items)
        .block(
            Block::default()
                .title("🛠️ Available Tools")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().bg(Color::DarkGray));

    let mut state = ListState::default();
    state.select(Some(app.selected_tool));
    f.render_stateful_widget(tools_list, area, &mut state);
}

fn render_system_info(f: &mut Frame, app: &App, area: Rect) {
    // CPU usage
    let cpu_usage = app.system.global_cpu_usage() as f64 / 100.0;
    let cpu_gauge = Gauge::default()
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .percent((cpu_usage * 100.0) as u16);

    // Memory usage
    let memory_usage = (app.system.used_memory() as f64 / app.system.total_memory() as f64) * 100.0;
    let memory_gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .percent(memory_usage as u16);

    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    f.render_widget(cpu_gauge, info_chunks[0]);
    f.render_widget(memory_gauge, info_chunks[1]);
}

fn render_security_status(f: &mut Frame, app: &App, area: Rect) {
    let mut security_text = vec![
        Line::from("🔒 Firewall: Active"),
        Line::from("🛡️ IDS: Monitoring"),
        Line::from("🚫 Blocked: 0 IPs"),
        Line::from("⚠️ Alerts: 0 new"),
    ];

    // Add error message if present
    if let Some(ref error) = app.last_error {
        security_text.push(Line::from(""));
        security_text.push(Line::from(Span::styled(
            format!("❌ Error: {}", error),
            Style::default().fg(Color::Red),
        )));
    }

    let security_panel = Paragraph::new(security_text)
        .block(
            Block::default()
                .title("🔐 Security Status")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(security_panel, area);
}
