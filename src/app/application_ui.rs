use crossterm::event::Event;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;

use crate::components::{
    BaseComponent, DrawableComponent,
    user_registration::UserRegistration
};

pub struct ApplicationUI {
    user_reg: UserRegistration
}

impl ApplicationUI {
    pub fn new() -> Self {
        ApplicationUI {
            user_reg: UserRegistration::new()
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
        self.user_reg.draw(f, area);
    }
}