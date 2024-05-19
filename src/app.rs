use std::{fs, io::Stderr};

use self::{
    key::handle_keypress,
    text::{Direction, TextBuffer},
};
use crate::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
mod key;
mod text;
pub enum SignalToApp {
    Exit,
    PutChar(char),
    MoveCursor(Direction),
    Backspace,
    Enter,
    Home,
    End,
}
pub struct App<'a> {
    terminal: Terminal<CrosstermBackend<Stderr>>,
    path: Option<&'a String>,
    buffer: TextBuffer,
}
impl<'a> App<'a> {
    pub fn new(
        terminal: Terminal<CrosstermBackend<Stderr>>,
        path: Option<&'a String>,
    ) -> Result<Self> {
        Ok(App {
            terminal,
            path,
            buffer: match path {
                Some(path) => fs::read_to_string(path)?.into(),
                None => TextBuffer::default(),
            },
        })
    }
    pub fn run(mut self) -> Result<()> {
        self.render()?;
        if let Some(signal) = handle_keypress()? {
            if let SignalToApp::Exit = signal {
                return self.exit();
            } else {
                self.handle_signal(signal)
            }
        }
        self.run()
    }
    fn handle_signal(&mut self, signal: SignalToApp) {
        match signal {
            SignalToApp::PutChar(ch) => self.buffer.put_char(ch),
            SignalToApp::MoveCursor(direction) => self.buffer.move_cursor(direction),
            SignalToApp::Backspace => self.buffer.backspace(),
            SignalToApp::Enter => self.buffer.enter(),
            SignalToApp::Home => self.buffer.home(),
            SignalToApp::End => self.buffer.end(),
            SignalToApp::Exit => panic!("This function can't handle exit"),
        }
    }
    fn exit(self) -> Result<()> {
        if let Some(path) = self.path {
            fs::write(path, self.buffer.to_string())?;
        }
        Ok(())
    }
    fn render(&mut self) -> Result<()> {
        // Render the UI
        self.terminal.draw(|f| {
            f.render_widget(self.buffer.paragraph(), f.size());
        })?;
        let cursor = self.buffer.cursor_pos();
        self.terminal.set_cursor(cursor.0, cursor.1)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
