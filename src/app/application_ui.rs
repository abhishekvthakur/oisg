use crossterm::event::Event;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use crate::components::{BaseComponent, DrawableComponent};

pub struct ApplicationUI {}

impl ApplicationUI {
    pub fn new() -> Self {
        ApplicationUI {}
    }
}

impl BaseComponent for ApplicationUI {
    fn event(&mut self, event: Event) -> Result<bool, ()> {
        Ok(true)
    }
}

impl DrawableComponent for ApplicationUI {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        todo!()
    }
}