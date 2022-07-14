use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event::Event;
use tui::{
    backend::Backend,
    Frame,
    layout::{
        Layout, Constraint, Direction, Rect
    }
};
use crate::{
    app::application_ui::ApplicationUI,
    components::{
        BaseComponent, Command, DrawableComponent
    },
    common::command_keys::CommandKeys,
    components::command::CommandComponent,
    components::user_registration::UserRegistration,
    db::models::UserInfo,
};

pub struct Application {
    ui: ApplicationUI,
    command: CommandComponent,
    user_registration: Option<RefCell<UserRegistration>>,
    command_keys: Rc<CommandKeys>,
    quit: bool,
}

impl Application {
    pub fn new(user_info: Option<UserInfo>) -> Self {
        let command_keys = Rc::new(CommandKeys::default());
        let user_registration = match user_info {
            None => Some(RefCell::new(UserRegistration::new(Rc::clone(&command_keys)))),
            Some(_) => None
        };

        let user_info: Rc<UserInfo> = match user_info {
            None => Rc::new(UserInfo::new()),
            Some(ui) => Rc::new(ui),
        };
        Application {
            ui: ApplicationUI::new(
                Rc::clone(&user_info),
                Rc::clone(&command_keys)
            ),
            command: CommandComponent::new(),
            user_registration,
            command_keys: Rc::clone(&command_keys),
            quit: false,
        }
    }

    pub fn is_quit(&self) -> bool {
        self.quit
    }

    fn get_common_commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();

        commands.push(Command {
            label: "Quit [^c]".to_string(),
            enable: true
        });

        commands
    }
}

impl BaseComponent for Application {
    fn event(&mut self, event: Event) -> Result<bool, ()> {
        if let Event::Key(ke) = event {
            return if ke == self.command_keys.quit {
                self.quit = true;
                Ok(true)
            } else {
                match &self.user_registration {
                    None => {
                        self.ui.event(event)
                    }
                    Some(user_registration) => {
                        user_registration.borrow_mut().event(event)
                    }
                }
            }
        }

        if let Event::Resize(_, _) = event {
            return Ok(true);
        }

        Ok(false)
    }
}

impl DrawableComponent for Application {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(2),
                Constraint::Length(1),
            ].as_ref())
            .split(area);

        let mut commands = Vec::new();
        commands.append(&mut self.get_common_commands());

        match &self.user_registration {
            None => {
                self.ui.draw(f, layout[0]);
                commands.append(&mut self.ui.get_commands());
            }
            Some(user_registration) => {
                user_registration.borrow_mut().draw(f, layout[0]);
                commands.append(&mut user_registration.borrow().get_commands());
            }
        };

        self.command.update_commands(commands);
        self.command.draw(f, layout[1]);
    }
}
