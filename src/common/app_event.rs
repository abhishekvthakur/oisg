use crossterm::event::Event;

#[derive(Debug, Copy, Clone)]
pub enum Notification {
    UserInfoSaved,
    UserJoined,
}

#[derive(Debug, Copy, Clone)]
pub enum AppEvent {
    InputEvent(Event),
    NotificationEvent(Notification),
    None
}
