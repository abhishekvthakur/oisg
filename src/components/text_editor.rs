use crossterm::event::{Event, poll};
use crate::BaseComponent;

/// `TextEditor` is a non drawable component
/// for handling text input operations
pub(crate) struct TextEditor {
    pub(crate) text: String,
    pub(crate) cur_pos: usize
}

impl TextEditor {
    pub fn from(text: String) -> Self {
        TextEditor {
            text,
            cur_pos: 0,
        }
    }

    pub fn new() -> Self {
        Self::from(String::new())
    }

    fn incr_cursor(&mut self) -> bool {
        if let Some(pos) = self.next_char_pos() {
            self.cur_pos = pos;

            return true
        }

        false
    }

    fn decr_cursor(&mut self) -> bool {
        if let Some(pos) = self.prev_char_pos() {
            self.cur_pos = pos;

            return true;
        }

        false
    }

    fn insert(&mut self, c: char) {
        self.text.insert(self.cur_pos, c);
        self.incr_cursor();
    }

    fn backspace(&mut self) -> bool {
        if let Some(pos) = self.prev_char_pos() {
            self.text.remove(pos);
            self.decr_cursor();

            return true
        }

        false
    }

    fn home(&mut self) -> bool {
        if self.cur_pos != 0 {
            self.cur_pos = 0;

            return true;
        }

        false
    }

    fn end(&mut self) -> bool {
        if self.cur_pos != self.text.len() {
            self.cur_pos = self.text.len();

            return true;
        }

        false
    }

    fn delete(&mut self) -> bool {
        if self.cur_pos < self.text.len() {
            self.text.remove(self.cur_pos);

            return true;
        }

        false
    }

    fn next_char_pos(&self) -> Option<usize> {
        if self.cur_pos == self.text.len() {
            return None;
        }

        let mut index = self.cur_pos.saturating_add(1);
        while index < self.text.len() && !self.text.is_char_boundary(index) {
            index += 1
        }

        Some(index)
    }

    fn prev_char_pos(&self) -> Option<usize> {
        if self.cur_pos == 0 {
            return None;
        }

        let mut index = self.cur_pos.saturating_sub(1);
        while index > 0 && !self.text.is_char_boundary(index) {
            index -= 1;
        }

        Some(index)
    }
}

impl BaseComponent for TextEditor {
    fn event(&mut self, event: Event) -> Result<bool, ()> {
        if let Event::Key(ke) = event {
            return match ke {
                _ => Ok(false)
            }
        }

        // no event consumed
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_pos_eng() {
        let mut text_editor = TextEditor::from(String::from("sample text"));

        assert_eq!(text_editor.next_char_pos(), Some(1));
        assert_eq!(text_editor.prev_char_pos(), None);

        text_editor.cur_pos = 4;
        assert_eq!(text_editor.next_char_pos(), Some(5));
        assert_eq!(text_editor.prev_char_pos(), Some(3));
    }

    #[test]
    fn test_char_pos_marathi() {
        let mut text_editor = TextEditor::from(String::from("नमुना मजकूर"));

        assert_eq!(text_editor.next_char_pos(), Some(3));
        assert_eq!(text_editor.prev_char_pos(), None);

        text_editor.cur_pos = 6;
        assert_eq!(text_editor.next_char_pos(), Some(9));
        assert_eq!(text_editor.prev_char_pos(), Some(3));
    }

    #[test]
    fn test_char_pos_chinese() {
        let mut text_editor = TextEditor::from(String::from("示例文本"));

        assert_eq!(text_editor.next_char_pos(), Some(3));
        assert_eq!(text_editor.prev_char_pos(), None);

        text_editor.cur_pos = 6;
        assert_eq!(text_editor.next_char_pos(), Some(9));
        assert_eq!(text_editor.prev_char_pos(), Some(3));
    }

    #[test]
    fn test_incr_cursor() {
        let mut text_editor = TextEditor::from(String::from("नमुना मजकूर"));
        assert_eq!(text_editor.next_char_pos(), Some(3));
        assert_eq!(text_editor.prev_char_pos(), None);

        text_editor.incr_cursor();
        assert_eq!(text_editor.next_char_pos(), Some(6));
        assert_eq!(text_editor.prev_char_pos(), Some(0));

        text_editor.incr_cursor();
        assert_eq!(text_editor.next_char_pos(), Some(9));
        assert_eq!(text_editor.prev_char_pos(), Some(3));

        assert_eq!(text_editor.incr_cursor(), true);
    }

    #[test]
    fn test_decr_cursor() {
        let mut text_editor = TextEditor::from(String::from("नमुना मजकूर"));
        text_editor.incr_cursor();
        text_editor.incr_cursor();

        text_editor.decr_cursor();
        assert_eq!(text_editor.next_char_pos(), Some(6));
        assert_eq!(text_editor.prev_char_pos(), Some(0));

        text_editor.decr_cursor();
        assert_eq!(text_editor.next_char_pos(), Some(3));
        assert_eq!(text_editor.prev_char_pos(), None);

        // no-op
        assert_eq!(text_editor.decr_cursor(), false)
    }

    #[test]
    fn test_insert() {
        let mut text_editor = TextEditor::from(String::from("नना मजकूर"));
        assert_eq!(text_editor.cur_pos, 0);
        assert_eq!(text_editor.text.len(), 25);

        assert_eq!(text_editor.incr_cursor(), true);
        text_editor.insert('म');
        assert_eq!(text_editor.cur_pos, 6);
        assert_eq!(text_editor.text.len(), 28);

        text_editor.insert('ु');
        assert_eq!(text_editor.cur_pos, 9);
        assert_eq!(text_editor.text.len(), 31);
        assert_eq!(text_editor.text, "नमुना मजकूर");
    }

    #[test]
    fn test_backspace() {
        let mut text_editor = TextEditor::from(String::from("नमुना मजकूर"));

        // no-op
        assert_eq!(text_editor.backspace(), false);

        text_editor.incr_cursor();
        text_editor.incr_cursor();

        assert_eq!(text_editor.backspace(), true);
        assert_eq!(text_editor.cur_pos, 3);
        assert_eq!(text_editor.text.len(), 28);

        assert_eq!(text_editor.backspace(), true);
        assert_eq!(text_editor.cur_pos, 0);
        assert_eq!(text_editor.text.len(), 25);
    }

    #[test]
    fn test_home() {
        let mut text_editor = TextEditor::from(String::from("नमुना मजकूर"));

        // no-op
        assert_eq!(text_editor.home(), false);

        assert_eq!(text_editor.incr_cursor(), true);
        assert_eq!(text_editor.incr_cursor(), true);
        assert_eq!(text_editor.cur_pos, 6);
        assert_eq!(text_editor.home(), true);
        assert_eq!(text_editor.cur_pos, 0);

        // no-op again
        assert_eq!(text_editor.home(), false);
    }

    #[test]
    fn test_end() {
        let mut text_editor = TextEditor::from(String::from("नमुना मजकूर"));

        assert_eq!(text_editor.end(), true);
        assert_eq!(text_editor.cur_pos, text_editor.text.len());

        // no-op
        assert_eq!(text_editor.end(), false);
    }

    #[test]
    fn test_delete() {
        let mut text_editor = TextEditor::from(String::from("नमुना मजकूर"));

        assert_eq!(text_editor.incr_cursor(), true);
        assert_eq!(text_editor.incr_cursor(), true);
        assert_eq!(text_editor.incr_cursor(), true);

        assert_eq!(text_editor.delete(), true);
        assert_eq!(text_editor.text.len(), 28);

        assert_eq!(text_editor.delete(), true);
        assert_eq!(text_editor.text.len(), 25);

        assert_eq!(text_editor.end(), true);
        // no-op
        assert_eq!(text_editor.delete(), false);
    }

    #[test]
    fn test_text_editing() {
        let mut text_editor = TextEditor::from(String::from("rigshit"));
        text_editor.end();
        text_editor.decr_cursor();
        text_editor.decr_cursor();
        text_editor.delete();
        text_editor.decr_cursor();
        text_editor.decr_cursor();
        text_editor.delete();

        assert_eq!(text_editor.text, "right");
    }

}