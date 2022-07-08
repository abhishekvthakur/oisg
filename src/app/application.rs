use std::rc::Rc;
use crossterm::event::Event;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use crate::app::application_ui::ApplicationUI;
use crate::components::{ BaseComponent, DrawableComponent };
use crate::common::command_keys::CommandKeys;

pub struct Application {
    ui: ApplicationUI,
    command_keys: Rc<CommandKeys>,
    quit: bool,
}

impl Application {
    pub fn new() -> Self {
        let command_keys = Rc::new(CommandKeys::default());

        Application {
            ui: ApplicationUI::new(Rc::clone(&command_keys)),
            command_keys: Rc::clone(&command_keys),
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
            return if ke == self.command_keys.quit {
                self.quit = true;
                Ok(true)
            } else {
                self.ui.event(event)
            }
        }

        Ok(false)
    }
}

impl DrawableComponent for Application {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        self.ui.draw(f, area);
    }
}