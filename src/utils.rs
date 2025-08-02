use std::time::Duration;

use crossterm::style::Stylize;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::ui::{
    Attributes, Color, ErrorMessageRenderConfig, IndexPrefix, RenderConfig, StyleSheet, Styled,
};

pub fn rcfg() -> RenderConfig<'static> {
    RenderConfig {
        prompt_prefix: Styled::new("?").with_fg(Color::DarkMagenta),
        answered_prompt_prefix: Styled::new("λ").with_fg(Color::DarkMagenta),
        prompt: StyleSheet::empty().with_attr(Attributes::BOLD),
        default_value: StyleSheet::empty().with_fg(Color::DarkGrey),
        placeholder: StyleSheet::empty(),
        help_message: StyleSheet::empty(),
        text_input: StyleSheet::empty(),
        error_message: ErrorMessageRenderConfig::empty()
            .with_prefix(Styled::new("#").with_fg(Color::DarkRed))
            .with_message(StyleSheet::new().with_fg(Color::DarkRed)),
        answer: StyleSheet::empty(),
        canceled_prompt_indicator: Styled::new("<canceled>"),
        password_mask: '*',
        highlighted_option_prefix: Styled::new(">").with_fg(Color::DarkMagenta),
        scroll_up_prefix: Styled::new(" "),
        scroll_down_prefix: Styled::new(" "),
        selected_checkbox: Styled::new("[x]"),
        unselected_checkbox: Styled::new("[ ]"),
        option_index_prefix: IndexPrefix::None,
        option: StyleSheet::empty().with_fg(Color::DarkGrey),
        selected_option: Some(StyleSheet::empty()),
    }
}

pub fn log_err(e: anyhow::Error) {
    eprintln!("{}", format!("# {}", e).dark_red());
}

pub fn log_info(msg: &str) {
    println!("{} {}", "λ".dark_magenta(), msg);
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
