use std::rc::Rc;
use crossterm::event::Event;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::components::{
    BaseComponent, DrawableComponent,
    user_registration::UserRegistration,
    Command,
    command::CommandComponent,
};
use crate::common::command_keys::CommandKeys;

pub struct ApplicationUI {
    command: CommandComponent,
    user_reg: UserRegistration
}

impl ApplicationUI {
    pub fn new(
        command_keys: Rc<CommandKeys>
    ) -> Self {
        ApplicationUI {
            command: CommandComponent::new(),
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
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(2),
                Constraint::Length(1),
            ].as_ref())
            .split(area);

        self.user_reg.draw(f, layout[0]);

        // TODO think something else, remove from draw, only
        let mut commands = self.user_reg.get_commands();
        commands.append(&mut self.get_commands());

        self.command.update_commands(commands);
        self.command.draw(f, layout[1]);
    }

    // default commands for application
    // like quit, help, etc
    fn get_commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();

        commands.push(Command {
            label: "Quit [^c]".to_string(),
            enable: true
        });

        commands
    }
}