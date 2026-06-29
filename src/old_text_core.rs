use ropey::Rope;

pub struct Editor {
    pub text: Rope,
    pub cursor_idx: usize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            text: Rope::from_str("хай из text_core.rs"),
            cursor_idx: 0,
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        self.text.insert(self.cursor_idx, &ch.to_string());
        self.cursor_idx += 1;
    }

    pub fn delete_backspace(&mut self) {
        if self.cursor_idx > 0 && self.text.len_chars() > 0 {
            self.text.remove((self.cursor_idx - 1)..self.cursor_idx);
            self.cursor_idx -= 1;
        }
    }

    pub fn insert_enter(&mut self) {
        self.text.insert(self.cursor_idx, "\n");
        self.cursor_idx += 1;
    }

    pub fn move_left(&mut self) {
        if self.cursor_idx > 0 {
            self.cursor_idx -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_idx < self.text.len_chars() {
            self.cursor_idx += 1;
        }
    }

    pub fn text_before_cursor_in_line(&self) -> String {
        let line = self.text.char_to_line(self.cursor_idx);
        let line_start = self.text.line_to_char(line);
        let col = self.cursor_idx - line_start;
        let line_str = self.text.line(line).to_string();
        line_str.chars().take(col).collect()
    }

    pub fn current_line(&self) -> usize {
        self.text.char_to_line(self.cursor_idx)
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = Rope::from_str(text);
        self.cursor_idx = 0;
    }

    pub fn get_text(&self) -> String {
        self.text.to_string()
    }
}
