use std::rc::Rc;
use tui::{
    Frame,
    backend::Backend,
    layout::Rect,
    widgets::{
        Block, Borders, BorderType, Paragraph
    }
};
use crate::{
    common::{
        command_keys::CommandKeys,
        app_event::AppEvent,
    },
    components::{
        BaseComponent, DrawableComponent,
    },
    db::models::UserInfo,
};

pub struct ApplicationUI {
    user_info: Rc<UserInfo>
}

impl ApplicationUI {
    pub fn new(
        user_info: Rc<UserInfo>,
        _command_keys: Rc<CommandKeys>
    ) -> Self {
        ApplicationUI {
            user_info,
        }
    }

    pub fn set_user_info(&mut self, user_info: Rc<UserInfo>) {
        self.user_info = Rc::clone(&user_info);
    }
}

impl BaseComponent for ApplicationUI {
    fn event(&mut self, event: AppEvent) -> Result<bool, ()> {
        Ok(false)
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
            self.user_info.user_name,
            self.user_info.user_id,
        )).block(block);

        f.render_widget(para, area);
    }
}
