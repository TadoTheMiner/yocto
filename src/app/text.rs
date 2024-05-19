use ratatui::widgets::Paragraph;
use Direction::*;

#[derive(Default, Clone)]
pub struct TextBuffer {
    text: Vec<String>,
    cursor: (u16, u16),
}

impl TextBuffer {
    pub fn backspace(&mut self) {
        if self.cursor.0 > 0 {
            let index = self.cursor.0.saturating_sub(1) as usize;
            self.current_line().remove(index);
            self.move_cursor(Direction::Left)
        } else if self.cursor.1 > 0 {
            self.text.remove(self.cursor.1 as usize);
            self.move_cursor(Direction::Up)
        }
    }
    pub fn paragraph(&self) -> Paragraph {
        Paragraph::new(self.to_string())
    }
    pub fn to_string(&self) -> String {
        self.text.join("\n")
    }
    pub fn enter(&mut self) {
        if (self.cursor.1 as usize) < self.text.len() {
            self.text.insert(self.cursor.1 as usize + 1, String::new());
        }

        self.text_lenght_to_cursor_position();
        self.cursor.1 += 1;
        self.cursor.0 = 0
    }

    fn text_lenght_to_cursor_position(&mut self) {
        while self.cursor.1 as usize >= self.text.len() {
            self.text.push(String::new())
        }
    }

    pub fn put_char(&mut self, ch: char) {
        let index = self.cursor.0 as usize;
        self.current_line().insert(index, ch);
        self.move_cursor(Direction::Right)
    }
    fn current_line(&mut self) -> &mut String {
        self.text_lenght_to_cursor_position();
        self.text.get_mut(self.cursor.1 as usize).unwrap()
    }
    pub fn cursor_pos(&self) -> (u16, u16) {
        self.cursor
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        if direction == Left && self.cursor.0 == 0 {
            self.move_cursor_non_wrapped(Up);
            self.end();
            return;
        }

        let current_line_lenght = self.current_line_lenght();
        if direction == Right && self.cursor.0 == current_line_lenght {
            self.move_cursor_non_wrapped(Down);
            self.end();
            return;
        }

        let should_go_to_end = self.cursor.0 == current_line_lenght;
        self.move_cursor_non_wrapped(direction);
        if let Up | Down = direction {
            if should_go_to_end {
                self.end();
                return;
            }
            self.cursor.0 = self.cursor.0.min(self.current_line_lenght())
        }
    }

    fn move_cursor_non_wrapped(&mut self, direction: Direction) {
        let cord_to_change = match direction {
            Left | Right => &mut self.cursor.0,
            _ => &mut self.cursor.1,
        };
        match direction {
            Left | Up => *cord_to_change = cord_to_change.saturating_sub(1),
            _ => *cord_to_change = cord_to_change.saturating_add(1),
        }
    }

    fn current_line_lenght(&mut self) -> u16 {
        self.current_line().len() as u16
    }

    pub fn home(&mut self) {
        self.cursor.0 = 0
    }

    pub fn end(&mut self) {
        self.cursor.0 = self.current_line_lenght()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<String> for TextBuffer {
    fn from(val: String) -> Self {
        TextBuffer {
            text: val.split('\n').map(String::from).collect(),
            cursor: (0, 0),
        }
    }
}
