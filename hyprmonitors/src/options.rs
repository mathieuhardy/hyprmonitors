use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Hyprland Monitor Manager")]
#[command(version = "1.0")]
#[command(about = "Configures monitors and workspaces according to profiles", long_about = None)]
pub struct Options {
    #[arg(short = 'p', long, value_name = "PROFILE_PATH")]
    pub profile: Option<PathBuf>,

    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
