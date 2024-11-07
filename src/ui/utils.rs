pub fn calculate_visible_lines(terminal_height: u16, port_terminal_ratio: f32) -> usize {
    let visible_pixels = (terminal_height as f32 * port_terminal_ratio).floor() as u16;
    visible_pixels as usize
}