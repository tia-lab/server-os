use anyhow::Result;
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
    process::{Command, Stdio},
    time::{Duration, Instant},
};
use sysinfo::System;

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

// Application state
struct App {
    tools: Vec<Tool>,
    current_tab: usize,
    selected_tool: usize,
    system: System,
    last_update: Instant,
}

impl App {
    fn new() -> Self {
        let tools = vec![
            Tool {
                name: "finder".to_string(),
                command: "xplr".to_string(),
                description: "Interactive file manager".to_string(),
                icon: "📁".to_string(),
                category: ToolCategory::Core,
            },
            Tool {
                name: "search".to_string(),
                command: "television".to_string(),
                description: "Fuzzy finder".to_string(),
                icon: "🔍".to_string(),
                category: ToolCategory::Core,
            },
            Tool {
                name: "disk".to_string(),
                command: "wiper".to_string(),
                description: "Disk analyzer".to_string(),
                icon: "💾".to_string(),
                category: ToolCategory::Core,
            },
            Tool {
                name: "system".to_string(),
                command: "bottom".to_string(),
                description: "System monitor".to_string(),
                icon: "📊".to_string(),
                category: ToolCategory::System,
            },
            Tool {
                name: "network".to_string(),
                command: "bandwhich".to_string(),
                description: "Network monitor".to_string(),
                icon: "🌐".to_string(),
                category: ToolCategory::System,
            },
            Tool {
                name: "trace".to_string(),
                command: "trippy".to_string(),
                description: "Network diagnostics".to_string(),
                icon: "📍".to_string(),
                category: ToolCategory::System,
            },
            Tool {
                name: "guard".to_string(),
                command: "heimdall".to_string(),
                description: "Intrusion detection".to_string(),
                icon: "🛡️".to_string(),
                category: ToolCategory::Security,
            },
            Tool {
                name: "firewall".to_string(),
                command: "dfw".to_string(),
                description: "Docker firewall".to_string(),
                icon: "🔥".to_string(),
                category: ToolCategory::Security,
            },
            Tool {
                name: "waf".to_string(),
                command: "aegis".to_string(),
                description: "Web application firewall".to_string(),
                icon: "🌐".to_string(),
                category: ToolCategory::Security,
            },
        ];

        let mut system = System::new_all();
        system.refresh_all();

        Self {
            tools,
            current_tab: 0,
            selected_tool: 0,
            system,
            last_update: Instant::now(),
        }
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

    fn launch_tool(&self) -> Result<()> {
        let tool = &self.tools[self.selected_tool];

        // Clear terminal
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

        println!("🚀 Launching {}...", tool.name);

        // Execute the tool
        let status = Command::new(&tool.command)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match status {
            Ok(_) => println!("✅ {} completed", tool.name),
            Err(e) => println!("❌ Failed to launch {}: {}", tool.name, e),
        }

        println!("Press Enter to return to OS dashboard...");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Restore terminal
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        Ok(())
    }

    fn update_system_info(&mut self) {
        if self.last_update.elapsed() > Duration::from_secs(1) {
            self.system.refresh_all();
            self.last_update = Instant::now();
        }
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
    let mut app = App::new();

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
                            eprintln!("Error launching tool: {}", e);
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
    let header = Paragraph::new("🛡️ server-os Dashboard")
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
    render_security_status(f, info_chunks[1]);

    // Footer
    let footer = Paragraph::new("↑↓/jk: Navigate | Enter: Launch | Tab: Switch | q/Esc: Quit")
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

fn render_security_status(f: &mut Frame, area: Rect) {
    let security_text = vec![
        Line::from("🔒 Firewall: Active"),
        Line::from("🛡️ IDS: Monitoring"),
        Line::from("🚫 Blocked: 0 IPs"),
        Line::from("⚠️ Alerts: 0 new"),
    ];

    let security_panel = Paragraph::new(security_text)
        .block(
            Block::default()
                .title("🔐 Security Status")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(security_panel, area);
}
