use std::fmt;

fn is_unicode_supported() -> bool {
    if cfg!(windows) {
        std::env::var("CI").is_ok()
            || std::env::var("WT_SESSION").is_ok()
            || std::env::var("TERM_PROGRAM").is_ok_and(|v| v == "vscode")
            || std::env::var("TERM").is_ok_and(|v| v == "xterm-256color" || v == "alacritty")
    } else {
        std::env::var("TERM").is_ok_and(|v| v != "dumb" && v != "linux")
    }
}

pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
    Success,
    Start,
}

impl Level {
    fn prefix(&self) -> String {
        let unicode = is_unicode_supported();
        match self {
            Level::Debug => {
                let sym = if unicode { "⚙" } else { "D" };
                format!("\x1b[90m{sym}\x1b[0m")
            }
            Level::Info => {
                let sym = if unicode { "ℹ" } else { "i" };
                format!("\x1b[36m{sym}\x1b[0m")
            }
            Level::Warn => "\x1b[43;30m WARN \x1b[0m".to_string(),
            Level::Error => "\x1b[41;30m ERROR \x1b[0m".to_string(),
            Level::Success => {
                let sym = if unicode { "✔" } else { "√" };
                format!("\x1b[32m{sym}\x1b[0m")
            }
            Level::Start => {
                let sym = if unicode { "◐" } else { "*" };
                format!("\x1b[35m{sym}\x1b[0m")
            }
        }
    }

    fn has_padding(&self) -> bool {
        matches!(self, Level::Warn | Level::Error)
    }
}

pub fn log(level: Level, args: fmt::Arguments<'_>) {
    if level.has_padding() {
        eprintln!("\n{} {}\n", level.prefix(), args);
    } else {
        eprintln!("{} {}", level.prefix(), args);
    }
}

pub fn blue(value: &dyn fmt::Display) -> String {
    format!("\x1b[34m{value}\x1b[0m")
}

pub fn dim(value: &dyn fmt::Display) -> String {
    format!("\x1b[2m{value}\x1b[0m")
}

macro_rules! logger_debug {
    ($($arg:tt)*) => { $crate::logger::log($crate::logger::Level::Debug, format_args!($($arg)*)) };
}

macro_rules! logger_info {
    ($($arg:tt)*) => { $crate::logger::log($crate::logger::Level::Info, format_args!($($arg)*)) };
}

macro_rules! logger_warn {
    ($($arg:tt)*) => { $crate::logger::log($crate::logger::Level::Warn, format_args!($($arg)*)) };
}

macro_rules! logger_error {
    ($($arg:tt)*) => { $crate::logger::log($crate::logger::Level::Error, format_args!($($arg)*)) };
}

macro_rules! logger_success {
    ($($arg:tt)*) => { $crate::logger::log($crate::logger::Level::Success, format_args!($($arg)*)) };
}

macro_rules! logger_start {
    ($($arg:tt)*) => { $crate::logger::log($crate::logger::Level::Start, format_args!($($arg)*)) };
}

pub(crate) use logger_debug;
pub(crate) use logger_error;
pub(crate) use logger_info;
pub(crate) use logger_start;
pub(crate) use logger_success;
pub(crate) use logger_warn;
