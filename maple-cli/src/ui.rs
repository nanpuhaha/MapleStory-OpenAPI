// src/ui.rs
//! ANSI terminal styling and formatting helpers for maple-cli.

pub const BOLD: &str = "\x1b[1m";
pub const BOLD_CYAN: &str = "\x1b[1;36m";
pub const BOLD_YELLOW: &str = "\x1b[1;33m";
pub const BOLD_GREEN: &str = "\x1b[1;32m";
pub const BOLD_RED: &str = "\x1b[1;31m";
pub const BOLD_WHITE: &str = "\x1b[1;37m";
pub const DIM: &str = "\x1b[2m";
pub const RESET: &str = "\x1b[0m";

/// Formats large numbers using Korean currency style (억 / 만).
pub fn format_number(n: i64) -> String {
    if n >= 100_000_000 {
        let eok = n / 100_000_000;
        let man = (n % 100_000_000) / 10_000;
        if man > 0 {
            format!("{}억 {:04}만", eok, man)
        } else {
            format!("{}억", eok)
        }
    } else if n >= 10_000 {
        format!("{}만", n / 10_000)
    } else {
        n.to_string()
    }
}

/// Formats diff with ANSI color (+ green, - red).
pub fn format_diff_colored(diff: i64) -> String {
    if diff > 0 {
        format!("{}+{}{}", BOLD_GREEN, format_number(diff), RESET)
    } else if diff < 0 {
        format!("{}-{}{}", BOLD_RED, format_number(diff.abs()), RESET)
    } else {
        format!("{}0{}", DIM, RESET)
    }
}

/// Formats diff with plain text (+ / -).
pub fn format_diff_plain(diff: i64) -> String {
    if diff > 0 {
        format!("+{}", format_number(diff))
    } else if diff < 0 {
        format!("-{}", format_number(diff.abs()))
    } else {
        "0".to_string()
    }
}

/// Formats mesos with Korean units.
pub fn format_mesos(mesos: u64) -> String {
    if mesos >= 100_000_000 {
        let eok = mesos / 100_000_000;
        let man = (mesos % 100_000_000) / 10_000;
        if man > 0 {
            format!("{}억 {:04}만 메소", eok, man)
        } else {
            format!("{}억 메소", eok)
        }
    } else if mesos >= 10_000 {
        format!("{}만 메소", mesos / 10_000)
    } else {
        format!("{} 메소", mesos)
    }
}
