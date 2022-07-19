use std::{
    thread,
    time::{
        Duration, Instant
    }
};
use crossterm::event;
use crossbeam_channel::{
    Sender, Receiver, unbounded
};
use crate::common::app_event::AppEvent;

const TICK_RATE: Duration = Duration::from_millis(200);

///
pub struct EventReceiver {
    receiver: Receiver<AppEvent>
}

impl EventReceiver {
    pub fn new() -> Self {
        let (tx_event, rx_event) = unbounded();
        Self::input_loop(tx_event);

        Self {
            receiver: rx_event
        }
    }

    fn input_loop(sender: Sender<AppEvent>) {
        let last_tick = Instant::now();

        thread::spawn(move || {
            loop {
                let timeout = TICK_RATE
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_millis(0));

                match event::poll(timeout) {
                    Ok(available) if available => {
                        if let Ok(evt) = event::read() {
                            let _ = sender.send(AppEvent::InputEvent(evt));
                        }
                    }
                    Err(_) => {},
                    _ => {}
                }
            }
        });
    }

    pub fn receiver(self) -> Receiver<AppEvent> {
        self.receiver
    }
}
