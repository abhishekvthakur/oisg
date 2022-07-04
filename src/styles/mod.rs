use tui::style::{
    Color, Style
};

pub fn cursor_style() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(Color::White)
}

pub fn placeholder_style() -> Style {
    Style::default()
        .fg(Color::Gray)
        .bg(Color::Black)
}