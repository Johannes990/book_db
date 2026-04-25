use ratatui::style::{Modifier, Style};

use crate::ui::colors::app_colors::AppColors;

pub struct AppStyles {
    // main section styles
    pub screen_style: Style,
    pub popup_style: Style,
    pub screen_border_style: Style,
    pub popup_border_style: Style,
    // info block
    pub info_style: Style,
    // warning and quit popups
    pub warning_style: Style,
    // error popup
    pub error_style: Style,
    // selected row style
    pub highlight_row_style: Style,
    // for every odd list element
    pub list_row_style: Style,
    // for every even list element
    pub list_row_alt_style: Style,
    // for styling identifiers (table names, col names, types etc):
    pub identifier_style: Style,
    // for metadata
    pub metadata_style: Style,
    // for singling out selected elemnt from various elements on the same row
    pub highlighted_element_style: Style,
    // footer rendering style
    pub footer_style: Style,
    pub footer_keycombo_style: Style,
}

impl AppStyles {
    pub fn from(colors: &AppColors) -> Self {
        let text = colors.text;
        let text_alt = colors.text_alt;
        let text_highlight = colors.text_highlight;
        let background = colors.background;
        let background_alt = colors.background_alt;
        let background_highlight = colors.background_highlight;
        let warning = colors.warning;
        let error = colors.error;
        let border = colors.border;
        let accent = colors.accent;

        Self {
            screen_style: Style::default().fg(text).bg(background),
            popup_style: Style::default().fg(text).bg(background_alt),
            screen_border_style: Style::default().fg(border).bg(background),
            popup_border_style: Style::default().fg(border).bg(background_alt),
            info_style: Style::default().fg(border).bg(background_alt),
            warning_style: Style::default().fg(text).bg(warning),
            error_style: Style::default().fg(text).bg(error),
            highlight_row_style: Style::default().fg(text_highlight).bg(background_highlight),
            list_row_style: Style::default().fg(text).bg(background),
            list_row_alt_style: Style::default().fg(text).bg(background_alt),
            identifier_style: Style::default()
                .fg(text)
                .add_modifier(Modifier::ITALIC | Modifier::UNDERLINED),
            metadata_style: Style::default().fg(text_alt).add_modifier(Modifier::ITALIC),
            highlighted_element_style: Style::default()
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            footer_style: Style::default().fg(text).bg(background_alt),
            footer_keycombo_style: Style::default().fg(accent).bg(background_alt),
        }
    }
}
