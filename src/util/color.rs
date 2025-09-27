use colored::{ColoredString, Colorize};

pub fn error(s: &str) -> ColoredString {
    s.red()
}

pub fn warn(s: &str) -> ColoredString {
    s.yellow()
}

pub fn info(s: &str) -> ColoredString {
    s.blue()
}

pub fn success(s: &str) -> ColoredString {
    s.green()
}

pub fn bold(s: &str) -> ColoredString {
    s.bold()
}
