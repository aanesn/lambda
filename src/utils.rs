use crossterm::style::Stylize;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::ui::{
    Attributes, Color, ErrorMessageRenderConfig, IndexPrefix, RenderConfig, StyleSheet, Styled,
};
use std::{path::PathBuf, time::Duration};

pub fn rcfg() -> RenderConfig<'static> {
    RenderConfig {
        prompt_prefix: Styled::new("?").with_fg(Color::DarkMagenta),
        answered_prompt_prefix: Styled::new("λ").with_fg(Color::DarkMagenta),
        prompt: StyleSheet::empty().with_attr(Attributes::BOLD),
        default_value: StyleSheet::empty().with_fg(Color::Grey),
        placeholder: StyleSheet::empty(),
        help_message: StyleSheet::empty(),
        text_input: StyleSheet::empty(),
        error_message: ErrorMessageRenderConfig::empty()
            .with_prefix(Styled::new("#").with_fg(Color::DarkRed))
            .with_message(StyleSheet::new().with_fg(Color::DarkRed)),
        answer: StyleSheet::empty().with_fg(Color::Grey),
        canceled_prompt_indicator: Styled::new("<canceled>"),
        password_mask: '*',
        highlighted_option_prefix: Styled::new(">").with_fg(Color::DarkMagenta),
        scroll_up_prefix: Styled::new(" "),
        scroll_down_prefix: Styled::new(" "),
        selected_checkbox: Styled::new("[x]"),
        unselected_checkbox: Styled::new("[ ]"),
        option_index_prefix: IndexPrefix::None,
        option: StyleSheet::empty().with_fg(Color::DarkGrey),
        selected_option: Some(StyleSheet::empty().with_fg(Color::Grey)),
    }
}

pub fn log_err(e: anyhow::Error) {
    eprintln!("{}", format!("# {}", e).dark_red());
}

pub fn log_info(primary: &str, secondary: &str) {
    println!("{} {} {}", "λ".dark_magenta(), primary, secondary.grey());
}

pub fn ms(dur: &Duration) -> String {
    format!("({:.1}ms)", dur.as_secs_f64() * 1000.0)
}

pub fn sec(dur: &Duration) -> String {
    format!("({:.1}s)", dur.as_secs_f64())
}

pub fn path(path: &PathBuf) -> String {
    format!("`{}`", path.display())
}

pub fn spinner() -> ProgressBar {
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
