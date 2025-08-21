use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
    Frame
};

use super::text_entry_field::TextEntryField;

pub struct TextForm {
    pub fields: Vec<TextEntryField>,
    pub index: usize,
    pub labels: Vec<String>,
    pub block_title: String,
    pub on_style: Style,
    pub off_style: Style,
    pub base_style: Style,
}

impl Widget for &TextForm {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.block_title.clone())
            .style(self.base_style);

        block.clone().render(area, buf);

        let inner = block.inner(area);
        let mut text = Text::default();

        for (i, field) in self.fields.iter().enumerate() {
            let mut line = format!("{}: ", self.labels[i]);
            if i == self.index {
                line.push_str(&field.text_value);
                text.push_line(Span::styled(line, self.on_style));
            } else {
                line.push_str(&field.text_value);
                text.push_line(Span::styled(line, self.off_style));
            }
        }

        let content = Paragraph::new(text).wrap(Wrap{ trim: false });

        content.render(inner, buf);
    }
}

impl TextForm {
    pub fn new(labels: Vec<String>, title: String) -> Self {
        let mut fields = Vec::new();
        for (i, _label) in labels.iter().enumerate() {
            fields.push(TextEntryField::default(String::new(), i == 0))
        }
        Self {
            fields,
            index: 0,
            labels,
            block_title: title,
            on_style: Style::default(),
            off_style: Style::default(),
            base_style: Style::default(),
        }
    }

    pub fn next(&mut self) {
        if !self.fields.is_empty() {
            self.fields[self.index].selected = false;
            self.index = (self.index + 1) % self.fields.len();
            self.fields[self.index].selected = true;
        }
    }

    pub fn previous(&mut self) {
        if !self.fields.is_empty() {
            self.fields[self.index].selected = false;
            self.index = if self.index == 0 {
                self.fields.len() - 1
            } else {
                self.index - 1
            };
            self.fields[self.index].selected = true;
        }
    }

    pub fn enter_char(&mut self, c: char) {
        if let Some(field) = self.fields.get_mut(self.index) {
            field.enter_char(c);
        }
    }

    pub fn pop_char(&mut self) {
        if let Some(field) = self.fields.get_mut(self.index) {
            field.pop_char();
        }
    }

    pub fn set_styles(&mut self, on_style: Style, off_style: Style, base_style: Style) {
        self.on_style = on_style;
        self.off_style = off_style;
        self.base_style = base_style;
    }

    pub fn update_cursor_pos(&self, frame: &mut Frame, area: Rect) {
        if let Some(active_field) = self.fields.get(self.index) {
            let cursor_x = area.x + self.labels[self.index].len() as u16 + 3 + active_field.cursor_pos as u16;
            let cursor_y = area.y + self.index as u16 + 1;
            frame.set_cursor_position((cursor_x, cursor_y));
        }
    }

    pub fn render_widget_and_cursor(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);
        frame.render_widget(self, area);
        self.update_cursor_pos(frame, area);
    }
}