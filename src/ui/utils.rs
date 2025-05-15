use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct ToggleButton<'a> {
    pub label: &'a str,
    pub active: bool,
    pub selected: bool,
    pub on_style: Style,
    pub off_style: Style,
    pub selected_border_style: Style,
}

impl<'a> Widget for ToggleButton<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.active { self.on_style } else { self.off_style };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(if self.selected {
                self.selected_border_style
            } else {
                Style::default()
            });
        let label = Line::from(self.label);
        let paragraph = Paragraph::new(label).style(style).block(block);

        paragraph.render(area, buf);
    }
}
