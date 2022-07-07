use tui::style::{
    Color, Style
};

pub fn cursor_style(focus: bool) -> Style {
    if !focus {
        return Style::default();
    }

    Style::default()
        .fg(Color::Black)
        .bg(Color::White)
}

pub fn input_style(focus: bool, placeholder: bool) -> Style {
    Style::default()
        .bg(match focus {
            true => Color::Rgb(40, 40, 40),
            false => Color::Black
        })
        .fg(match placeholder {
            true => Color::Gray,
            false => Color::White
        })
}

pub fn command_style(enable: bool) -> Style {
    Style::default()
        .bg(Color::Blue)
        .fg(match enable {
            true =>Color::White,
            false => Color::Rgb(90, 90, 90)
        })
}