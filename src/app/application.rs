use std::{
    cell::RefCell,
    rc::Rc,
};
use crossterm::event::Event;
use crossbeam_channel::Sender;
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
    common::{
        command_keys::CommandKeys,
        app_event::AppEvent,
    },
    components::command::CommandComponent,
    components::user_registration::UserRegistration,
    db::{
        self,
        models::UserInfo
    },
};
use crate::common::app_event::Notification;

pub struct Application {
    ui: ApplicationUI,
    command: CommandComponent,
    user_registration: Option<RefCell<UserRegistration>>,
    command_keys: Rc<CommandKeys>,
    quit: bool,
}

impl Application {
    pub fn new(
        user_info: Option<UserInfo>, tx_notification: Sender<AppEvent>
    ) -> Self {
        let command_keys = Rc::new(CommandKeys::default());
        let user_registration = match user_info {
            None => Some(RefCell::new(UserRegistration::new(
                Rc::clone(&command_keys),
                tx_notification.clone()
            ))),
            Some(_) => None
        };

        let user_info: Rc<UserInfo> = match user_info {
            None => Rc::new(UserInfo::new()),
            Some(ui) => Rc::new(ui),
        };
        Application {
            ui: ApplicationUI::new(
                Rc::clone(&user_info),
                Rc::clone(&command_keys),
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

    pub fn disable_user_registration(&mut self) {
        self.user_registration = None;
    }
}

impl BaseComponent for Application {
    fn event(&mut self, event: AppEvent) -> Result<bool, ()> {
        if let AppEvent::NotificationEvent(notification) = event {
            return match notification {
                Notification::UserInfoSaved => {
                    self.user_registration = None;
                    let user_info = db::operations::get_user_info().unwrap().unwrap();
                    self.ui.set_user_info(Rc::new(user_info));

                    Ok(true)
                }
                _ => Ok(false)
            }
        } else if let AppEvent::InputEvent(evt) = event {
            if let Event::Key(ke) = evt {
                return if ke == self.command_keys.quit {
                    self.quit = true;
                    Ok(false)
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
            } else if let Event::Resize(_, _) = evt {
                return Ok(true);
            }
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
