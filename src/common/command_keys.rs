use crossterm::event::{ KeyEvent, KeyCode, KeyModifiers };

pub struct CommandKeys {
    // common keys
    pub quit: KeyEvent,
    pub help: KeyEvent,

    // user registration
    pub focus_next: KeyEvent,
    pub clear: KeyEvent,
    pub next: KeyEvent,
    pub save: KeyEvent,
}

impl CommandKeys {
    pub fn default() -> Self {
        CommandKeys {
            quit: KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            help: KeyEvent::new(KeyCode::Char('h'), KeyModifiers::CONTROL),
            focus_next: KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
            clear: KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
            next: KeyEvent::new(KeyCode::Char('n'), KeyModifiers::CONTROL),
            save: KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        }
    }
}