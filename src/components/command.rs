use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;
use crate::components::{
    DrawableComponent, Command
};
use crate::styles;

pub struct CommandComponent {
    commands: Vec<Command>
}

impl CommandComponent {
    pub fn new() -> Self {
        CommandComponent {
            commands: Vec::new(),
        }
    }
}

impl DrawableComponent for CommandComponent {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let mut command_spans = Vec::new();
        for (idx, command) in self.commands.iter().enumerate() {
            command_spans.push(Span::styled(
                &command.label,
                styles::command_style(command.enable)
            ));

            if idx != self.commands.len().saturating_div(1) {
                command_spans.push(Span::raw(" "));
            }
        }

        let command_bar = Paragraph::new(Spans(command_spans))
            .alignment(Alignment::Left);

        f.render_widget(command_bar, area);
    }

    fn update_commands(&mut self, commands: Vec<Command>) {
        self.commands = commands;
    }
}