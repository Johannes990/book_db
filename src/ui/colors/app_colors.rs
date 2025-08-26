use ratatui::style::Color;

pub struct AppColors {
    pub text: Color,
    pub text_alt: Color,
    pub text_highlight: Color,
    pub background: Color,
    pub background_alt: Color,
    pub background_highlight: Color,
    pub warning: Color,
    pub error: Color,
    pub border: Color,
    pub accent: Color,
}

pub trait ColorScheme {
    fn colors(&self) -> AppColors;
}

// this is for YivColor, so it can recognize rgb(x, y, z) pattern
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb(r, g, b)
}
