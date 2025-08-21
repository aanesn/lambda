use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub(crate) fn spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.magenta} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(120));
    pb
}
