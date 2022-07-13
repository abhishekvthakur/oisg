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
use crate::{
    common::command_keys::CommandKeys,
    components::{
        BaseComponent, DrawableComponent,
        user_registration::UserRegistration,
    }
};

pub struct ApplicationUI {
    user_reg: UserRegistration
}

impl ApplicationUI {
    pub fn new(
        command_keys: Rc<CommandKeys>
    ) -> Self {
        ApplicationUI {
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

        f.render_widget(block, area);
    }
}
