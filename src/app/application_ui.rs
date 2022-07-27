use std::rc::Rc;
use tui::{
    Frame,
    backend::Backend,
    layout::{
        Rect, Constraint, Direction, Layout
    },
    widgets::{
        Block, Borders, BorderType
    }
};
use crate::{
    common::{
        command_keys::CommandKeys,
        app_event::AppEvent,
    },
    components::{
        BaseComponent, DrawableComponent,
        userinfo::UserInfoComponent,
        text_input::TextInput
    },
    db::models::UserInfo,
    styles,
};

pub struct ApplicationUI {
    user_info: Rc<UserInfo>,
    message_input: TextInput
}

impl ApplicationUI {
    pub fn new(
        user_info: Rc<UserInfo>,
        _command_keys: Rc<CommandKeys>
    ) -> Self {
        let mut message_input = TextInput::with_placeholder(
            "type message...".to_string(),
        );
        message_input.set_focus(true);

        ApplicationUI {
            user_info,
            message_input,
        }
    }

    pub fn set_user_info(&mut self, user_info: Rc<UserInfo>) {
        self.user_info = Rc::clone(&user_info);
    }
}

impl BaseComponent for ApplicationUI {
    fn event(&mut self, event: AppEvent) -> Result<bool, ()> {
        self.message_input.event(event)
    }
}

impl DrawableComponent for ApplicationUI {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let hor_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref())
            .split(area);

        let ver_split_1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),
                Constraint::Min(10)
            ].as_ref())
            .split(hor_split[0]);

        let ver_split_2 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(4)
            ].as_ref())
            .split(hor_split[1]);

        let mut userinfo_comp = UserInfoComponent::new(Rc::clone(&self.user_info));
        let userinfo = Block::default()
            .title("User Info")
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(styles::border_style(false));
        let inner_rect = userinfo.inner(ver_split_1[0]);

        f.render_widget(userinfo, ver_split_1[0]);
        userinfo_comp.draw(f, inner_rect);

        let list = Block::default()
            .title("List")
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(styles::border_style(false));
        f.render_widget(list, ver_split_1[1]);

        let conversation = Block::default()
            .title("Conversation")
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(styles::border_style(false));
        f.render_widget(conversation, ver_split_2[0]);

        let input = Block::default()
            .title("Input")
            .border_type(BorderType::Plain)
            .borders(Borders::ALL);

        let message_rect = input.inner(ver_split_2[1]);

        f.render_widget(input, ver_split_2[1]);
        self.message_input.draw(f, message_rect);
    }
}
