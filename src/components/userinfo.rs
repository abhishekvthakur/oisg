use std::rc::Rc;
use tui::{
    backend::Backend,
    Frame,
    layout::{
        Layout, Rect, Direction, Constraint
    },
};
use tui::widgets::Paragraph;
use crate::{
    db::models::UserInfo,
    DrawableComponent,
    styles
};

pub struct UserInfoComponent {
    user_info: Rc<UserInfo>
}

impl UserInfoComponent {
    pub fn new(user_info: Rc<UserInfo>) -> Self {
        UserInfoComponent {
            user_info
        }
    }
}

impl DrawableComponent for UserInfoComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let ver_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1)
            ].as_ref())
            .horizontal_margin(1)
            .split(area);

        let user_name = Paragraph::new(
            self.user_info.user_name.as_str()
        ).style(styles::user_name_style());
        let user_id = Paragraph::new(
            self.user_info.user_id.as_str()
        ).style(styles::user_id_style());;

        f.render_widget(user_name, ver_layout[0]);
        f.render_widget(user_id, ver_layout[1]);
    }
}