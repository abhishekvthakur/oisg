use crossterm::event::Event;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::components::{
    BaseComponent, DrawableComponent,
    user_registration::UserRegistration,
    command::CommandComponent,
};

pub struct ApplicationUI {
    command: CommandComponent,
    user_reg: UserRegistration
}

impl ApplicationUI {
    pub fn new() -> Self {
        ApplicationUI {
            command: CommandComponent::new(),
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
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(2),
                Constraint::Length(1),
            ].as_ref())
            .split(area);

        self.user_reg.draw(f, layout[0]);

        self.command.update_commands(self.user_reg.get_commands());
        self.command.draw(f, layout[1]);
    }
}