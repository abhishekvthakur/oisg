use std::{
    io,
    rc::Rc
};
use crossterm::event::Event;
use crossbeam_channel::Sender;
use tui::{
    backend::Backend,
    Frame,
    layout::{
        Constraint, Direction, Layout, Rect
    },
    widgets::{
        Block, Borders, BorderType, Clear, Paragraph
    }
};
use names;
use crate::{
    components::{
        BaseComponent, DrawableComponent, Command,
        text_input::TextInput
    },
    common::{
        self,
        command_keys::CommandKeys,
        app_event::{ AppEvent, Notification }
    },
    styles,
    db
};

pub struct UserRegistration {
    name: TextInput,
    userid: TextInput,
    command_keys: Rc<CommandKeys>,
    tx_notification: Sender<AppEvent>,
    focus: bool,
    err_msg: Option<String>
}

impl UserRegistration {
    pub fn new(
        command_keys: Rc<CommandKeys>,
        tx_notification: Sender<AppEvent>
    ) -> Self {
        Self::edit(
            command_keys,
            String::new(),
            tx_notification
        )
    }

    pub fn edit(
        command_keys: Rc<CommandKeys>,
        text: String,
        tx_notification: Sender<AppEvent>
    ) -> Self {
        let mut name = TextInput::with(
            text,
            "Enter name...".to_string(),
        );
        name.set_focus(true);

        let userid = TextInput::with(
            Self::get_next_name(),
            "Enter userid...".to_string()
        );

        UserRegistration {
            name,
            userid,
            command_keys: Rc::clone(&command_keys),
            tx_notification,
            err_msg: Some("Please enter name".to_string()),
            focus: true
        }
    }

    fn get_next_name() -> String {
        let mut generator = names::Generator::with_naming(names::Name::Numbered);

        generator.next().unwrap()
    }

    fn focus_next(&mut self) {
        if self.name.is_focus() {
            self.name.set_focus(false);
            self.userid.set_focus(true);
        } else {
            self.name.set_focus(true);
            self.userid.set_focus(false);
        }
    }

    fn set_next_user_id(&mut self) {
        self.userid.clear();
        self.userid.set_text(Self::get_next_name());
    }

    fn validate_fields(&mut self) {
        let name_text = self.name.get_text();
        let userid_text = self.userid.get_text();

        if name_text.trim().len() > 0 && userid_text.trim().len() > 0 {
            self.err_msg = None;
            return;
        }

        if name_text.trim().len() == 0 {
            self.err_msg = Some("Please enter name".to_string());
        } else if userid_text.trim().len() == 0 {
            self.err_msg = Some("Please enter userid".to_string());
        }
    }

    fn save_user_details(&self) -> io::Result<()> {
        db::operations::save_user_details(
            self.name.get_text().to_string(),
            self.userid.get_text().to_string()
        )
    }
}

impl BaseComponent for UserRegistration {
    fn event(&mut self, event: AppEvent) -> Result<bool, ()> {
        if !self.focus {
            return Ok(false);
        }

        if let AppEvent::InputEvent(evt) = event {
            if let Event::Key(ke) = evt {
                return if ke == self.command_keys.focus_next {
                    self.focus_next();

                    Ok(true)
                } else if ke == self.command_keys.clear {
                    if self.name.is_focus() {
                        self.name.clear();
                    } else {
                        self.userid.clear();
                    }

                    self.validate_fields();
                    Ok(true)
                } else if ke == self.command_keys.next {
                    self.set_next_user_id();

                    self.validate_fields();
                    Ok(true)
                } else if ke == self.command_keys.save {
                    if self.err_msg == None {
                        return match self.save_user_details() {
                            Ok(_) => {
                                let event = AppEvent::NotificationEvent(Notification::UserInfoSaved);
                                let _ = self.tx_notification.send(event);

                                Ok(true)
                            },
                            Err(_) => Err(())
                        }
                    }

                    Ok(true)
                } else {
                    let result = if self.name.is_focus() {
                        self.name.event(event)
                    } else {
                        self.userid.event(event)
                    };

                    return match result {
                        Ok(consumed) if consumed => {
                            self.validate_fields();
                            Ok(true)
                        },
                        _ => Ok(false)
                    }
                }
            }
        }

        Ok(false)
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }

    fn is_focus(&self) -> bool {
        self.focus
    }
}

impl DrawableComponent for UserRegistration {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let block = Block::default()
            .title("User Registration")
            .borders(Borders::ALL)
            .border_type(BorderType::Thick);

        let mut center_area = common::get_center_rect_absolute(70, 6, area);
        if self.err_msg == None {
            center_area = Rect::new(
                center_area.x, center_area.y,
                center_area.width, center_area.height.saturating_sub(1)
            )
        }
        let inner_area = block.inner(center_area);

        let ver_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ].as_ref())
            .horizontal_margin(1)
            .split(inner_area);

        let hor_layout_1 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(85),
            ].as_ref())
            .split(ver_layout[0]);

        let hor_layout_2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(85),
            ].as_ref())
            .split(ver_layout[2]);

        f.render_widget(Clear, center_area);
        f.render_widget(block, center_area);

        f.render_widget(Paragraph::new("Name   :"), hor_layout_1[0]);
        f.render_widget(Paragraph::new("Userid :"), hor_layout_2[0]);

        self.name.draw(f, hor_layout_1[1]);
        self.userid.draw(f, hor_layout_2[1]);

        if let Some(err_message) = &self.err_msg {
            f.render_widget(
                Paragraph::new(err_message.as_str()).style(styles::error_msg_style()),
                ver_layout[3]
            );
        } else {
            f.render_widget(Clear, ver_layout[3]);
        }
    }

    fn get_commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();

        commands.push(Command {
            label: "Focus Next [⇥]".to_string(),
            enable: true
        });

        commands.push(Command {
            label: "Clear [^r]".to_string(),
            enable: true
        });

        commands.push(Command {
            label: "Next [^n]".to_string(),
            enable: self.userid.is_focus()
        });

        commands.push(Command {
            label: "Save [⏎]".to_string(),
            enable: self.err_msg == None
        });

        commands
    }
}
