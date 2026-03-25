// https://github.com/nik-rev/simply-colored/blob/main/src/lib.rs

pub const BOLD: &str = "\x1b[1m";

pub const BLUE: &str = "\x1b[94m";
pub const CYAN: &str = "\x1b[96m";
pub const GREEN: &str = "\x1b[92m";
pub const MAGENTA: &str = "\x1b[95m";
pub const RED: &str = "\x1b[91m";
pub const WHITE: &str = "\x1b[97m";
pub const YELLOW: &str = "\x1b[93m";

pub const RESET: &str = "\x1b[0m";

pub fn log_monitor_status(monitor: &str, enabled: bool) {
    if enabled {
        println!("{BOLD}{YELLOW}monitor{RESET}:{monitor}:{BOLD}{GREEN}enabled{RESET}");
    } else {
        println!("{BOLD}{YELLOW}monitor{RESET}:{monitor}:{BOLD}{RED}disabled{RESET}");
    }
}

pub fn log_workspace_assignment(id: usize, monitor: &str, persistent: bool, default: bool) {
    let default_str = if default {
        format!("{BOLD}{CYAN}:default ")
    } else {
        String::new()
    };

    let persistent_str = if persistent {
        format!("{BOLD}{WHITE}:persistent")
    } else {
        String::new()
    };

    println!(
        "{BOLD}{YELLOW}monitor{RESET}:{monitor} {BOLD}{MAGENTA}workspace{RESET}:{id:>02}{persistent_str}{default_str}{RESET}"
    );
}

pub fn log_workspace_move(id: usize, monitor: &str) {
    println!(
        "{BOLD}{YELLOW}monitor{RESET}:{monitor} {BOLD}{MAGENTA}workspace{RESET}:{id:>02}:{BOLD}{BLUE}moved{RESET}"
    );
}

pub fn log_jump_to_workspace(id: usize) {
    println!("{BOLD}{MAGENTA}workspace{RESET}:{id:>02}:{BOLD}{BLUE}selected{RESET}");
}
