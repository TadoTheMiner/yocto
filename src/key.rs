use crossterm::event::{self, Event, KeyCode};

use crate::{text::Direction, SignalToApp, TextBuffer};

pub fn handle_keypress(buffer: &mut TextBuffer) -> Option<SignalToApp> {
    if let Ok(Event::Key(key)) = event::read() {
        match key.code {
            KeyCode::Backspace => buffer.backspace(),
            KeyCode::Enter => buffer.enter(),
            KeyCode::Left => buffer.move_cursor(Direction::Left),
            KeyCode::Right => buffer.move_cursor(Direction::Right),
            KeyCode::Up => buffer.move_cursor(Direction::Up),
            KeyCode::Down => buffer.move_cursor(Direction::Down),
            KeyCode::Home => buffer.home(),
            KeyCode::End => buffer.end(),
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            KeyCode::Tab => {}
            KeyCode::BackTab => {}
            KeyCode::Delete => {}
            KeyCode::Insert => {}
            KeyCode::F(_) => {}
            KeyCode::Char(ch) => buffer.put_char(ch),
            KeyCode::Null => {}
            KeyCode::Esc => return Some(SignalToApp::Exit),
            KeyCode::CapsLock => {}
            KeyCode::ScrollLock => {}
            KeyCode::NumLock => {}
            KeyCode::PrintScreen => {}
            KeyCode::Pause => {}
            KeyCode::Menu => {}
            KeyCode::KeypadBegin => {}
            KeyCode::Media(_) => {}
            KeyCode::Modifier(_) => {}
        }
    }
    None
}
