use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::widgets::text_box::TextBox;

#[derive(Clone)]
pub struct TextEntryField {
    pub text_box: TextBox,
    pub selected: bool,
    pub on_style: Style,
    pub off_style: Style,
}

impl Widget for TextEntryField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.selected {
            self.on_style
        } else {
            self.off_style
        };
        let block = Block::default().borders(Borders::NONE);
        let text = Line::from(self.text_box.text_value);
        let paragraph = Paragraph::new(text).style(style).block(block);

        paragraph.render(area, buf);
    }
}

impl Widget for &TextEntryField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.selected {
            self.on_style
        } else {
            self.off_style
        };

        Paragraph::new(Line::from(self.text_box.text_value.clone()))
            .style(style)
            .block(Block::default().borders(Borders::NONE))
            .render(area, buf);
    }
}

impl TextEntryField {
    pub fn from(text_value: String, selected: bool) -> Self {
        TextEntryField {
            text_box: TextBox::new(text_value),
            selected,
            on_style: Style::default(),
            off_style: Style::default(),
        }
    }

    pub fn default() -> Self {
        TextEntryField {
            text_box: TextBox::default(),
            selected: false,
            on_style: Style::default(),
            off_style: Style::default(),
        }
    }

    pub fn set_on_style(&mut self, style: Style) {
        self.on_style = style;
    }

    pub fn set_off_style(&mut self, style: Style) {
        self.off_style = style;
    }

    pub fn cursor_position(&self, area: Rect) -> Option<Position> {
        if self.selected {
            Some(Position::new(
                area.x + self.text_box.cursor_pos as u16,
                area.y, // adjust if you want +1
            ))
        } else {
            None
        }
    }
}
