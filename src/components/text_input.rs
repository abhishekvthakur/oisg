use tui::{
    backend::Backend,
    Frame,
    layout::Rect,
    text::{
        Span, Spans
    },
    widgets::Paragraph
};
use crate::{
    styles,
    components::{
        BaseComponent, DrawableComponent,
        text_editor::TextEditor
    },
    common::app_event::AppEvent
};

pub struct TextInput {
    editor: TextEditor,
    placeholder: String,
    focus: bool,
}

impl TextInput {
    pub fn new() -> Self {
        Self::with_placeholder(Self::default_placeholder())
    }

    pub fn with_placeholder(placeholder: String) -> Self {
        TextInput {
            editor: TextEditor::new(),
            placeholder,
            focus: false,
        }
    }

    pub fn with_text(text: String) -> Self {
        TextInput {
            editor: TextEditor::from(text),
            placeholder: Self::default_placeholder(),
            focus: false,
        }
    }

    pub fn with(text: String, placeholder: String) -> Self {
        TextInput {
            editor: TextEditor::from(text),
            placeholder,
            focus: false,
        }
    }

    fn default_placeholder() -> String {
        String::from("type something...")
    }

    pub fn get_text(&self) -> &str {
        self.editor.text.as_str()
    }

    pub fn set_text(&mut self, text: String) {
        self.editor.text = text;
    }

    pub fn clear(&mut self) {
        self.editor.text.clear();
        self.editor.cur_pos = 0;
    }

    fn get_draw_text(&self) -> Option<Vec<Span>> {
        // no text
        if self.get_text().len() ==  0 {
            return None
        }

        let mut texts = Vec::new();
        if self.editor.cur_pos > 0 {
            texts.push(Span::raw(self.editor.text[0..self.editor.cur_pos].to_owned()))
        }

        // for showing cursor
        let cursor_text = self.editor.next_char_pos()
            .map_or(" ".to_owned(), |pos| {
                self.editor.text[self.editor.cur_pos..pos].to_owned()
            });

        texts.push(Span::styled(cursor_text, styles::cursor_style(self.focus)));

        // add remaining text, if any
        if let Some(pos) = self.editor.next_char_pos() {
            if pos < self.editor.text.len() {
                texts.push(Span::raw(self.editor.text[pos..].to_owned()))
            }
        }

        Some(texts)
    }
}

impl BaseComponent for TextInput {
    fn event(&mut self, event: AppEvent) -> Result<bool, ()> {
        if !self.focus {
            return Ok(false);
        }

        self.editor.event(event)
    }

    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }

    fn is_focus(&self) -> bool {
        self.focus
    }
}

impl DrawableComponent for TextInput {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let paragraph = match self.get_draw_text() {
            Some(texts) => {
                Paragraph::new(Spans::from(texts))
                    .style(styles::input_style(self.focus, false))
            },
            None => {
                Paragraph::new(Span::raw(self.placeholder.as_str()))
                    .style(styles::input_style(self.focus, true))
            }
        };

        f.render_widget(paragraph, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_text_input() {
        let text_input = TextInput::new();

        assert_eq!(text_input.focus, false);
        assert_eq!(text_input.editor.text.len(), 0);
        assert_eq!(text_input.placeholder, TextInput::default_placeholder());
        assert_eq!(text_input.editor.cur_pos, 0);
    }

    #[test]
    fn test_text_input_with_placeholder() {
        let ph = String::from("");
        let text_input = TextInput::with_placeholder(ph.to_owned());

        assert_eq!(text_input.focus, false);
        assert_eq!(text_input.editor.text.len(), 0);
        assert_eq!(text_input.placeholder, ph);
        assert_eq!(text_input.editor.cur_pos, 0);
    }

    #[test]
    fn test_text_input_with_text() {
        let text = String::from("sample text");
        let text_input = TextInput::with_text(text.to_owned());

        assert_eq!(text_input.focus, false);
        assert_eq!(text_input.get_text().len(), text.len());
        assert_eq!(text_input.get_text(), text);
        assert_eq!(text_input.editor.cur_pos, 0);
    }
}