mod components;
mod app;
mod styles;
mod common;
mod constants;
mod db;

use std::{
    error::Error,
    io,
};
use crossterm::{
    ExecutableCommand,
    terminal::{
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    },
};
use tui::{
    backend::CrosstermBackend,
    terminal::Terminal
};
use crossbeam_channel::{
    Select, Receiver,
    unbounded
};
use crate::{
    components::{
        BaseComponent, DrawableComponent
    },
    common::app_event::AppEvent,
    app::event_receiver::EventReceiver
};

fn main() -> Result<(), Box<dyn Error>> {
    // ensuring db exists, if not create one
    db::ensure_db_exists()?;

    // setup terminal for drawing
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;

    // init terminal with backend
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let (tx_notification, rx_notification) = unbounded::<AppEvent>();
    let event_receiver = EventReceiver::new();
    let rx_input = event_receiver.receiver();

    // create application
    let mut application = app::application::Application::new(
        db::operations::get_user_info()?,
        tx_notification
    );

    let mut first_draw = true;

    loop {
        if first_draw {
            terminal.draw(|f| application.draw(f, f.size()))?;
            first_draw = false;
            continue;
        }

        let event = select_event([
            &rx_input,
            &rx_notification
        ].as_ref())?;

        // ignore if there is no event
        if let AppEvent::None = event {
            continue;
        }

        match application.event(event) {
            Ok(consumed) if consumed => {
                terminal.draw(|f| application.draw(f, f.size()))?;
            },
            Err(_) => {
            }
            _ => {}
        }

        if application.is_quit() {
            break;
        }
    }

    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}

fn select_event(receivers: &[&Receiver<AppEvent>]) -> io::Result<AppEvent> {
    let mut select = Select::new();
    for receiver in receivers {
        select.recv(receiver);
    }

    let operation = select.select();
    let index = operation.index();

    match operation.recv(&receivers[index]) {
        Ok(evt) => Ok(evt),
        Err(_) => Ok(AppEvent::None)
    }
}
