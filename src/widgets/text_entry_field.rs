use ratatui::{
    buffer::Buffer, 
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget}
};

#[derive(Clone)]
pub struct TextEntryField {
    pub text_value: String,
    pub selected: bool,
    pub on_style: Style,
    pub off_style: Style,
}

impl Widget for TextEntryField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.selected { self.on_style } else { self.off_style };
        let block = Block::default()
            .borders(Borders::NONE);
        let text = Line::from(self.text_value);
        let paragraph = Paragraph::new(text).style(style).block(block);

        paragraph.render(area, buf);
    }
}

impl Widget for &TextEntryField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.selected { self.on_style } else { self.off_style };

        Paragraph::new(Line::from(self.text_value.clone()))
            .style(style)
            .block(Block::default().borders(Borders::NONE))
            .render(area, buf);
    }
}

impl TextEntryField {
    pub fn default(text_value: String, selected: bool) -> Self {
        TextEntryField {
            text_value,
            selected,
            on_style: Style::default(),
            off_style: Style::default(),
        }
    }

    pub fn enter_char(&mut self, c: char) {
        self.text_value.push(c);
    }

    pub fn pop_char(&mut self) {
        self.text_value.pop();
    }

    pub fn set_on_style(&mut self, style: Style) {
        self.on_style = style;
    }

    pub fn set_off_style(&mut self, style: Style) {
        self.off_style = style;
    }
}
