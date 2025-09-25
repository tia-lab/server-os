use anyhow::Result;
use std::env;

#[cfg(feature = "bandwhich")]
use bandwhich;

#[cfg(feature = "bottom")]
use bottom;


/// Integrated tool launcher that uses built-in tool functions instead of external commands
pub struct IntegratedTools;

impl IntegratedTools {
    /// Launch a tool by name using the integrated crate functions
    pub fn launch_tool(tool_name: &str, args: &[String]) -> Result<()> {
        match tool_name {
            "trace" => {
                #[cfg(feature = "trippy")]
                {
                    // For now, launch trippy as external command until we can integrate properly
                    // This is a transitional approach
                    Self::launch_external("trippy", args)
                }
                #[cfg(not(feature = "trippy"))]
                {
                    Err(anyhow::anyhow!("Trippy not available - feature disabled"))
                }
            }
            "network" => {
                #[cfg(feature = "bandwhich")]
                {
                    // Launch bandwhich using integrated library function
                    bandwhich::launch_bandwhich(args)
                        .map_err(|e| anyhow::anyhow!("Bandwhich error: {}", e))
                }
                #[cfg(not(feature = "bandwhich"))]
                {
                    Err(anyhow::anyhow!(
                        "Bandwhich not available - feature disabled"
                    ))
                }
            }
            "guard" => {
                #[cfg(feature = "heimdall")]
                {
                    // For now, launch heimdall as external command until we can integrate properly
                    Self::launch_external("heimdall", args)
                }
                #[cfg(not(feature = "heimdall"))]
                {
                    Err(anyhow::anyhow!("Heimdall not available - feature disabled"))
                }
            }
            "firewall" => {
                #[cfg(feature = "dfw")]
                {
                    // For now, launch dfw as external command until we can integrate properly
                    Self::launch_external("dfw", args)
                }
                #[cfg(not(feature = "dfw"))]
                {
                    Err(anyhow::anyhow!("DFW not available - feature disabled"))
                }
            }
            "waf" => {
                #[cfg(feature = "aegis")]
                {
                    // For now, launch aegis as external command until we can integrate properly
                    Self::launch_external("aegis", args)
                }
                #[cfg(not(feature = "aegis"))]
                {
                    Err(anyhow::anyhow!("Aegis not available - feature disabled"))
                }
            }
            "finder" => {
                #[cfg(feature = "yazi-fm")]
                {
                    // Use Yazi file manager
                    let args_vec = args.to_vec();
                    yazi_fm::launch_yazi_sync(args_vec)
                        .map_err(|e| anyhow::anyhow!("Yazi error: {}", e))
                }
                #[cfg(not(feature = "yazi-fm"))]
                {
                    Err(anyhow::anyhow!("Yazi not available - feature disabled"))
                }
            }
            "system" => {
                #[cfg(feature = "bottom")]
                {
                    bottom::launch_bottom(args.to_vec())
                        .map_err(|e| anyhow::anyhow!("Bottom error: {}", e))
                }
                #[cfg(not(feature = "bottom"))]
                {
                    Err(anyhow::anyhow!("Bottom not available - feature disabled"))
                }
            }
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name)),
        }
    }

    /// Launch an external command - fallback for tools not yet integrated
    fn launch_external(command: &str, args: &[String]) -> Result<()> {
        use std::process::{Command, Stdio};

        let mut cmd = Command::new(command);
        cmd.args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let status = cmd.status()?;

        if !status.success() {
            return Err(anyhow::anyhow!(
                "Tool '{}' exited with code: {:?}",
                command,
                status.code()
            ));
        }

        Ok(())
    }

    /// Check if a tool is available (compiled with the right features)
    pub fn is_tool_available(tool_name: &str) -> bool {
        match tool_name {
            "trace" => cfg!(feature = "trippy"),
            "network" => cfg!(feature = "bandwhich"),
            "guard" => cfg!(feature = "heimdall"),
            "firewall" => cfg!(feature = "dfw"),
            "waf" => cfg!(feature = "aegis"),
            "finder" => cfg!(feature = "yazi-fm") || true, // Yazi or built-in file manager
            "system" => cfg!(feature = "bottom"),
            _ => false,
        }
    }

    /// Check if a command exists in PATH
    fn command_exists(cmd: &str) -> bool {
        if let Ok(path) = env::var("PATH") {
            for dir in path.split(':') {
                let full_path = format!("{}/{}", dir, cmd);
                if std::path::Path::new(&full_path).exists() {
                    return true;
                }
            }
        }
        false
    }
}
