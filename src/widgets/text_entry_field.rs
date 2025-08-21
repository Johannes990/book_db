use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Clone)]
pub struct TextEntryField {
    pub text_value: String,
    pub cursor_pos: usize,
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
        let text = Line::from(self.text_value);
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
            cursor_pos: 0,
            selected,
            on_style: Style::default(),
            off_style: Style::default(),
        }
    }

    pub fn enter_char(&mut self, c: char) {
        self.text_value.push(c);
        self.move_cursor_right();
    }

    pub fn pop_char(&mut self) {
        self.text_value.pop();
        self.move_cursor_left();
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
                area.x + self.cursor_pos as u16,
                area.y, // adjust if you want +1
            ))
        } else {
            None
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_pos.saturating_sub(1);
        self.cursor_pos = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_pos.saturating_add(1);
        self.cursor_pos = self.clamp_cursor(cursor_moved_right);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.text_value.chars().count())
    }
}
