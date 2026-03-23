// https://github.com/nik-rev/simply-colored/blob/main/src/lib.rs

pub const BOLD: &str = "\x1b[1m";

pub const BLUE: &str = "\x1b[94m";
pub const CYAN: &str = "\x1b[96m";
pub const GREEN: &str = "\x1b[92m";
pub const MAGENTA: &str = "\x1b[95m";
pub const RED: &str = "\x1b[91m";
pub const YELLOW: &str = "\x1b[93m";

pub const RESET: &str = "\x1b[0m";

pub fn log_monitor_status(monitor: &str, enabled: bool) {
    if enabled {
        println!("{BOLD}{YELLOW}monitor{RESET}:{monitor}:{BOLD}{RED}disabled{RESET}");
    } else {
        println!("{BOLD}{YELLOW}monitor{RESET}:{monitor}:{BOLD}{GREEN}enabled{RESET}");
    }
}

pub fn log_workspace_assignment(id: u64, monitor: &str, default: bool) {
    let default_str = if default {
        format!("{BOLD}{CYAN}:default ")
    } else {
        String::new()
    };

    println!(
        "{BOLD}{YELLOW}monitor{RESET}:{monitor} {BOLD}{MAGENTA}workspace{RESET}:{id}{default_str}{RESET}"
    );
}

pub fn log_workspace_move(id: u64, monitor: &str) {
    println!(
        "{BOLD}{YELLOW}monitor{RESET}:{monitor} {BOLD}{MAGENTA}workspace{RESET}:{id}:{BOLD}{BLUE}moved{RESET}"
    );
}

pub fn log_jump_to_workspace(id: u64) {
    println!("{BOLD}{MAGENTA}workspace{RESET}:{id}:{BOLD}{BLUE}selected{RESET}");
}
