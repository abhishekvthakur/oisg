mod text_editor;
pub mod text_input;
pub mod user_registration;
pub mod command;

use crossterm::event::{Event, KeyEvent};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;

// These traits borrowed from gitui

/// `BaseComponent` trait defines the methods that should be
/// declare in component
pub trait BaseComponent {
    fn event(&mut self, event: Event) -> Result<bool, ()>;

    fn set_focus(&mut self, focus: bool) {
        // unimplemented!()
    }

    fn is_focus(&self) -> bool {
        false
    }
}

/// `DrawableComponent` should be implemented by components
/// that need to be draw on terminal
pub trait DrawableComponent {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect
    );

    fn get_commands(&self) -> Vec<Command> {
        vec![]
    }

    #[allow(unused)]
    fn update_commands(&mut self, commands: Vec<Command>) {}
}

pub struct Command {
    label: String,
    shortcut: KeyEvent,
    enable: bool,
}