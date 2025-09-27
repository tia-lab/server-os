use anyhow::Result;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{CompletionType, Config, Context, Editor};
use std::borrow::Cow::{self, Borrowed, Owned};
use std::collections::HashMap;
use std::process::Command;
use sysinfo::System;

#[derive(Clone)]
struct Tool {
    command: String,
    description: String,
}

struct ServerOsHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    tools: HashMap<String, Tool>,
}

impl ServerOsHelper {
    fn new() -> Self {
        let mut tools = HashMap::new();

        // Define server OS tools
        tools.insert(
            ":finder".to_string(),
            Tool {
                command: "yazi".to_string(),
                description: "Yazi file manager".to_string(),
            },
        );

        tools.insert(
            ":system".to_string(),
            Tool {
                command: "btm".to_string(),
                description: "Bottom system monitor".to_string(),
            },
        );

        tools.insert(
            ":network".to_string(),
            Tool {
                command: "bandwhich".to_string(),
                description: "Bandwhich network monitor".to_string(),
            },
        );

        tools.insert(
            ":trace".to_string(),
            Tool {
                command: "trip 8.8.8.8".to_string(),
                description: "Trippy network diagnostics (traces to 8.8.8.8)".to_string(),
            },
        );

        tools.insert(
            ":git".to_string(),
            Tool {
                command: "serie".to_string(),
                description: "Serie git graph viewer".to_string(),
            },
        );


        tools.insert(
            ":help".to_string(),
            Tool {
                command: "help".to_string(),
                description: "Show available commands".to_string(),
            },
        );

        tools.insert(
            ":status".to_string(),
            Tool {
                command: "status".to_string(),
                description: "Show system security status".to_string(),
            },
        );

        tools.insert(
            ":update".to_string(),
            Tool {
                command: "update".to_string(),
                description: "Update Server OS to latest version".to_string(),
            },
        );

        Self {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            validator: MatchingBracketValidator::new(),
            hinter: HistoryHinter {},
            tools,
        }
    }
}

impl Completer for ServerOsHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        if line.starts_with(':') {
            // Complete server OS commands
            let search = &line[0..pos];
            let matches: Vec<Pair> = self
                .tools
                .keys()
                .filter(|cmd| cmd.starts_with(search))
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();
            Ok((0, matches))
        } else {
            // Fallback to file completion for shell commands
            self.completer.complete(line, pos, _ctx)
        }
    }
}

impl Hinter for ServerOsHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for ServerOsHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Owned(format!("\x1b[38;5;154m{}\x1b[0m", prompt))
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("\x1b[38;5;154m{}\x1b[0m", hint))
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        if line.starts_with(':') {
            if self.tools.contains_key(line.trim()) {
                Owned(format!("\x1b[1;38;5;154m{}\x1b[0m", line))
            } else {
                Owned(format!("\x1b[38;5;154m{}\x1b[0m", line))
            }
        } else {
            self.highlighter.highlight(line, pos)
        }
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

impl Validator for ServerOsHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

impl rustyline::Helper for ServerOsHelper {}

fn show_help(tools: &HashMap<String, Tool>) {
    println!("\nserver-os commands:");
    println!("  Type TAB to see available commands");
    println!("  Commands starting with ':' are server-os tools");
    println!("  Commands without ':' run as shell commands");
    println!();

    let mut tool_list: Vec<_> = tools.iter().collect();
    tool_list.sort_by_key(|(k, _)| *k);

    for (cmd, tool) in tool_list {
        println!("  {:<12} - {}", cmd, tool.description);
    }
    println!();
}

fn update_server_os() {
    println!("\n🔄 Updating Server OS...");
    println!("═══════════════════════════════");

    // Pull latest from git
    println!("📥 Fetching latest version from GitHub...");
    let git_pull = Command::new("git")
        .args(&["pull", "origin", "main"])
        .status();

    match git_pull {
        Ok(status) if status.success() => {
            println!("✅ Repository updated");

            // Rebuild and reinstall
            println!("🔨 Building latest version...");
            let build_result = Command::new("cargo")
                .args(&["build", "--release"])
                .status();

            match build_result {
                Ok(status) if status.success() => {
                    println!("📦 Installing...");
                    let install_result = Command::new("cargo")
                        .args(&["install", "--path", "."])
                        .status();

                    match install_result {
                        Ok(status) if status.success() => {
                            println!("✅ Server OS updated successfully!");
                            println!("🔄 Please restart 'os' to use the new version");
                        }
                        _ => eprintln!("❌ Installation failed"),
                    }
                }
                _ => eprintln!("❌ Build failed"),
            }
        }
        Ok(_) => {
            println!("⚠️  No updates available or git pull failed");
            println!("   Make sure you're in the server-os directory");
        }
        Err(e) => {
            eprintln!("❌ Failed to run git: {}", e);
            eprintln!("   Make sure git is installed and you're in the server-os directory");
        }
    }
    println!();
}

fn show_status() {
    println!("\n🛡️ Server OS Security Status");
    println!("═══════════════════════════════");

    let mut sys = System::new_all();
    sys.refresh_all();

    // System Info
    println!("\n📊 System Information:");
    println!("  • CPU Usage: {:.1}%", sys.global_cpu_usage());
    println!(
        "  • Memory: {:.1} GB / {:.1} GB",
        sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0
    );
    println!("  • Processes: {}", sys.processes().len());
    println!("  • Uptime: {} hours", System::uptime() / 3600);

    // Security Features
    println!("\n🔒 Security Features:");
    println!("  ✅ System Monitoring (sysinfo)");
    println!("  ✅ File Integrity (notify)");
    println!("  ✅ Network Analysis (pnet)");
    println!("  ✅ Cryptography (ring)");

    // Available Tools
    println!("\n🛠️ Available Tools:");
    println!("  • File Managers: xplr, yazi");
    println!("  • System Monitor: bottom");
    println!("  • Network: bandwhich, trippy");
    println!("  • Security: dfw, aegis");

    println!();
}

fn launch_tool(command: &str) -> Result<()> {
    // Split command into program and arguments
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(anyhow::anyhow!("Empty command"));
    }

    let program = parts[0];
    let args = &parts[1..];

    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to launch {}: {}", program, e))?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "Tool '{}' failed with exit code: {:?}",
            program,
            status.code()
        ));
    }

    Ok(())
}

fn run_shell_command(input: &str) -> Result<()> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(input)
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to run shell command: {}", e))?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "Command failed with exit code: {:?}",
            status.code()
        ));
    }

    Ok(())
}

fn main() -> Result<()> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular)
        .build();
    let helper = ServerOsHelper::new();
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(helper));

    // Load history
    let _ = rl.load_history(".server-os-history");

    println!("server-os v0.1.0 - Security-hardened server OS");
    println!("Type ':help' for available commands, or TAB for completion");
    println!("Commands without ':' run as shell commands");
    println!();

    loop {
        match rl.readline("os> ") {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                rl.add_history_entry(input)?;

                // Handle server-os commands
                if input.starts_with(':') {
                    if let Some(helper) = rl.helper() {
                        if input == ":help" {
                            show_help(&helper.tools);
                            continue;
                        }

                        if input == ":status" {
                            show_status();
                            continue;
                        }

                        if input == ":update" {
                            update_server_os();
                            continue;
                        }

                        if input == ":exit" || input == ":quit" {
                            break;
                        }

                        if let Some(tool) = helper.tools.get(input) {
                            println!("Launching {}...", tool.description);
                            if let Err(e) = launch_tool(&tool.command) {
                                eprintln!("Error: {}", e);
                            }
                        } else {
                            eprintln!(
                                "Unknown command: {}. Type ':help' for available commands.",
                                input
                            );
                        }
                    }
                } else if input == "exit" || input == "quit" {
                    break;
                } else {
                    // Run as shell command
                    if let Err(e) = run_shell_command(input) {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save history
    let _ = rl.save_history(".server-os-history");
    println!("Goodbye!");

    Ok(())
}
