use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::widgets::{
    Block, Borders, BorderType, Paragraph
};
use crate::{
    AppEvent, styles
};
use crate::components::{
    BaseComponent, DrawableComponent
};

pub struct ChatArea {
}

impl ChatArea {
    pub fn new() -> Self {
        ChatArea{}
    }
}

impl BaseComponent for ChatArea {
    fn event(&mut self, event: AppEvent) -> Result<bool, ()> {
        todo!()
    }
}

impl DrawableComponent for ChatArea {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let conversation = Block::default()
            .title("Conversation")
            .border_type(BorderType::Plain)
            .borders(Borders::ALL)
            .border_style(styles::border_style(false));

        let paragraph = Paragraph::new("Some Text").block(conversation);

        f.render_widget(paragraph, area);
    }
}