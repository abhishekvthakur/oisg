use tui::style::{
    Color, Style
};

pub fn cursor_style() -> Style {
    Style::default()
        .fg(Color::White)
        .bg(Color::Black)
}

pub fn placeholder_style() -> Style {
    Style::default()
        .fg(Color::Gray)
        .bg(Color::Black)
}