use crossterm::event::{
    Event, KeyCode, KeyModifiers
};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use crate::app::application_ui::ApplicationUI;
use crate::components::{
    BaseComponent, DrawableComponent
};

pub struct Application {
    ui: ApplicationUI,
    quit: bool,
}

impl Application {
    pub fn new() -> Self {
        Application {
            ui: ApplicationUI::new(),
            quit: false,
        }
    }

    pub fn is_quit(&self) -> bool {
        self.quit
    }
}

impl BaseComponent for Application {
    fn event(&mut self, event: Event) -> Result<bool, ()> {
        if let Event::Key(ke) = event {
            return match ke.code {
                KeyCode::Char('c') if ke.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.quit = true;
                    Ok(true)
                },
                _ => self.ui.event(event)
            }
        }

        Ok(true)
    }
}
