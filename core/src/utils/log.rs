use crossterm::style::Stylize;
use std::{path::PathBuf, time::Duration};

pub fn err(e: anyhow::Error) {
    eprintln!("{}", format!("# {}", e).dark_red());
}

pub(crate) fn info(msg: &str, desc: &str) {
    println!("{} {} {}", "λ".dark_magenta(), msg, desc.grey());
}

pub(crate) fn ms(msg: &str, dur: &Duration) {
    info(msg, &format!("({:.1}ms)", dur.as_secs_f64() * 1000.0))
}

pub(crate) fn sec(msg: &str, dur: &Duration) {
    info(msg, &format!("({:.1}s)", dur.as_secs_f64()))
}

pub(crate) fn path(msg: &str, path: &PathBuf) {
    info(msg, &format!("`{}`", path.display()))
}
