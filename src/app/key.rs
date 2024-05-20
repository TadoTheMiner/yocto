use super::{text::Direction, SignalToApp};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode::*};
pub fn handle_keypress() -> Result<Option<SignalToApp>> {
    match event::read()? {
        Event::Key(key) => Ok(match key.code {
            Backspace => Some(SignalToApp::Backspace),
            Enter => Some(SignalToApp::Enter),
            Left => Some(SignalToApp::MoveCursor(Direction::Left)),
            Right => Some(SignalToApp::MoveCursor(Direction::Right)),
            Up => Some(SignalToApp::MoveCursor(Direction::Up)),
            Down => Some(SignalToApp::MoveCursor(Direction::Down)),
            Home => Some(SignalToApp::Home),
            End => Some(SignalToApp::End),
            PageUp => None,
            PageDown => None,
            Tab => None,
            BackTab => None,
            Delete => None,
            Insert => None,
            F(_) => None,
            Char(ch) => Some(SignalToApp::PutChar(ch)),
            Null => None,
            Esc => Some(SignalToApp::Exit),
            CapsLock => None,
            ScrollLock => None,
            NumLock => None,
            PrintScreen => None,
            Pause => None,
            Menu => None,
            KeypadBegin => None,
            Media(_) => None,
            Modifier(_) => None,
        }),
        _ => Ok(None),
    }
}
