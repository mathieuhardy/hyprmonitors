// https://github.com/nik-rev/simply-colored/blob/main/src/lib.rs

pub const BOLD: &str = "\x1b[1m";

pub const BG_BLUE: &str = "\x1b[104m";
pub const BG_GREEN: &str = "\x1b[102m";
pub const BG_MAGENTA: &str = "\x1b[105m";
pub const BG_RED: &str = "\x1b[101m";
pub const BG_YELLOW: &str = "\x1b[103m";
pub const BG_WHITE: &str = "\x1b[107m";

pub const BLACK: &str = "\x1b[90m";

pub const RESET: &str = "\x1b[0m";

pub fn log_monitor_status(monitor: &str, enabled: bool) {
    if enabled {
        println!("{BOLD}{BLACK}{BG_GREEN} {monitor:>5} {RESET}");
    } else {
        println!("{BOLD}{BLACK}{BG_RED} {monitor:>5} {RESET}");
    }
}

pub fn log_workspace_assignment(id: u64, monitor: &str, default: bool) {
    let default_str = if default {
        format!("{BG_YELLOW} default ")
    } else {
        format!("{BG_WHITE}         ")
    };

    println!("{BOLD}{BLACK}{BG_GREEN} {monitor:>5} {BG_BLUE} WS {id:>02} {default_str}{RESET}");
}

pub fn log_workspace_move(id: u64, monitor: &str) {
    println!(
        "{BOLD}{BLACK}{BG_GREEN} {monitor:>5} {BG_BLUE} WS {id:>02} {BG_YELLOW}   moved {RESET}"
    );
}

pub fn log_jump_to_workspace(id: u64) {
    println!("{BOLD}{BLACK}{BG_MAGENTA} WS {id:>02} {RESET}");
}
