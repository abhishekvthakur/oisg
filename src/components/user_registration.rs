use crossterm::event::{Event, KeyCode, KeyModifiers};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::widgets::{
    Block, Borders, BorderType, Clear
};
use crate::{
    components::{
        BaseComponent, DrawableComponent,
        text_input::TextInput
    },
    common
};

pub struct UserRegistration {
    name_field: TextInput,
    focus: bool
}

impl UserRegistration {
    pub fn new() -> Self {
        let mut name_field = TextInput::with_placeholder(
            "Enter your name...".to_owned()
        );
        name_field.set_focus(true);

        UserRegistration {
            name_field,
            focus: true
        }
    }

    pub fn edit(text: String) -> Self {
        let mut name_field = TextInput::with(
            text,
            "Enter your name...".to_owned(),
        );
        name_field.set_focus(true);

        UserRegistration {
            name_field,
            focus: true
        }
    }

    pub fn reset(&mut self) {
        self.name_field.clear();
    }
}

impl BaseComponent for UserRegistration {
    fn event(&mut self, event: Event) -> Result<bool, ()> {
        if !self.focus {
            return Ok(false);
        }

        if let Event::Key(ke) = event {
            match ke.code {
                KeyCode::Char(c) if ke.modifiers.contains(KeyModifiers::CONTROL) => {
                    // TODO Remove hardcoding, map short keys
                    if c == 'r' || c == 'R' {
                        self.name_field.clear();
                        return Ok(true);
                    }
                },
                _ => return self.name_field.event(event)
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

        // let center_area = common::get_center_rect(10, 50, area);
        let center_area = common::get_center_rect_absolute(70, 3, area);
        let ti_area = block.inner(center_area);

        f.render_widget(Clear, center_area);
        f.render_widget(block, center_area);
        self.name_field.draw(f, ti_area);
    }
}
