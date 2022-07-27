use tui::style::{Color, Modifier, Style};

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

pub fn error_msg_style() -> Style {
    Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK)
}

pub fn user_name_style() -> Style {
    Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
}

pub fn user_id_style() -> Style {
    Style::default()
        .fg(Color::Gray)
        .add_modifier(Modifier::BOLD)
}

pub fn border_style(focus: bool) -> Style {
    Style::default()
        .fg(if focus {
            Color::White
        } else {
            Color::Gray
        })
}