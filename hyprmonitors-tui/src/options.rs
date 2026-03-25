use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Hyprland Monitor TUI")]
#[command(version = "1.0")]
#[command(about = "Configures monitors and workspaces", long_about = None)]
pub struct Options {}
