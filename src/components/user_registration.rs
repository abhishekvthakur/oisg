use std::rc::Rc;
use crossterm::event::Event;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{
    Constraint, Direction, Layout, Rect
};
use tui::widgets::{
    Block, Borders, BorderType, Clear, Paragraph
};
use names;
use crate::{
    components::{
        BaseComponent, DrawableComponent, Command,
        text_input::TextInput
    },
    common::{ self, command_keys::CommandKeys }
};

pub struct UserRegistration {
    name: TextInput,
    userid: TextInput,
    command_keys: Rc<CommandKeys>,
    focus: bool
}

impl UserRegistration {
    pub fn new(command_keys: Rc<CommandKeys>) -> Self {
        Self::edit(command_keys, String::new())
    }

    pub fn edit(command_keys: Rc<CommandKeys>, text: String) -> Self {
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
}

impl BaseComponent for UserRegistration {
    fn event(&mut self, event: Event) -> Result<bool, ()> {
        if !self.focus {
            return Ok(false);
        }

        if let Event::Key(ke) = event {
            return if ke == self.command_keys.focus_next {
                self.focus_next();

                Ok(true)
            } else if ke == self.command_keys.clear {
                if self.name.is_focus() {
                    self.name.clear();
                } else {
                    self.userid.clear();
                }

                Ok(true)
            } else if ke == self.command_keys.next {
                self.set_next_user_id();

                Ok(true)
            } else {
                if self.name.is_focus() {
                    self.name.event(event)
                } else {
                    self.userid.event(event)
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

        let center_area = common::get_center_rect_absolute(70, 5, area);
        let inner_area = block.inner(center_area);

        let ver_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
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
            enable: false
        });

        commands
    }
}
