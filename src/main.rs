use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::{
    ExecutableCommand,
    terminal::{
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    },
    event
};
use crossterm::event::Event;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal for drawing
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;

    // wait for use to press any key
    loop {
        let timeout = Duration::from_millis(200);

        if let Ok(_) = event::poll(timeout) {
            let event = event::read()?;
            if let Event::Key(_) = event {
                break;
            }
        }
    }

    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
