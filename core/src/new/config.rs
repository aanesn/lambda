use inquire::ui::{
    Attributes, Color, ErrorMessageRenderConfig, IndexPrefix, RenderConfig, StyleSheet, Styled,
};

pub(crate) fn rcfg() -> RenderConfig<'static> {
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
