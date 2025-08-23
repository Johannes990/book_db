use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct SelectableLine {
    pub label: String,
    pub active: bool,
    pub on_style: Style,
    pub off_style: Style,
}

impl Widget for SelectableLine {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.active {
            self.on_style
        } else {
            self.off_style
        };
        let block = Block::default().borders(Borders::NONE);
        let label = Line::from(self.label);
        let paragraph = Paragraph::new(label).style(style).block(block);

        paragraph.render(area, buf);
    }
}

impl SelectableLine {
    pub fn default(
        label: &str,
        active: bool,
        selected: bool,
        on_style: Style,
        off_style: Style
    ) -> Self {
        let prefix = if selected { "* " } else { "  " };
        let postfix = if active { "ON" } else { "OFF" };
        let label = format!("{}{}{}", prefix, label, postfix);
        SelectableLine {
            label,
            active,
            on_style,
            off_style,
        }
    }
}
