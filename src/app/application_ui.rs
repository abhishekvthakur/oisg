use std::rc::Rc;
use crossterm::event::Event;
use tui::{
    Frame,
    backend::Backend,
    layout::Rect,
    widgets::{
        Block, Borders, BorderType
    }
};
use tui::widgets::Paragraph;
use crate::{
    common::command_keys::CommandKeys,
    components::{
        BaseComponent, DrawableComponent,
        user_registration::UserRegistration,
    },
    db::models::UserInfo,
};

pub struct ApplicationUI {
    user_info: Rc<UserInfo>,
    user_reg: UserRegistration
}

impl ApplicationUI {
    pub fn new(
        user_info: Rc<UserInfo>,
        command_keys: Rc<CommandKeys>
    ) -> Self {
        ApplicationUI {
            user_info,
            user_reg: UserRegistration::new(Rc::clone(&command_keys))
        }
    }
}

impl BaseComponent for ApplicationUI {
    fn event(&mut self, event: Event) -> Result<bool, ()> {
        self.user_reg.event(event)
    }
}

impl DrawableComponent for ApplicationUI {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let block = Block::default()
            .title("Sample Block")
            .border_type(BorderType::Thick)
            .borders(Borders::ALL);

        let para = Paragraph::new(format!(
            "{}, {}",
            self.user_info.get("USER_NAME").unwrap(),
            self.user_info.get("USER_ID").unwrap(),
        )).block(block);

        f.render_widget(para, area);
    }
}
