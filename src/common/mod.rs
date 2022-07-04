use tui::{
    layout::{ Rect, Layout, Direction }
};
use tui::layout::Constraint;

pub fn get_center_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let ver_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ].as_ref())
        .split(area);

    let hor_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ].as_ref())
        .split(ver_layout[1]);

    hor_layout[1]
}

pub fn get_center_rect_absolute(
    width: u16,
    height: u16,
    r: Rect
) -> Rect {
    Rect::new(
        r.width.saturating_sub(width) / 2,
        r.height.saturating_sub(height) / 2,
        width,
        height,
    )
}
