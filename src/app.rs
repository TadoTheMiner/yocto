use std::{fs, io::Stderr};

use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};

use crate::{key, text::TextBuffer, Result};

pub enum SignalToApp {
    Exit,
}

pub fn run(mut terminal: Terminal<CrosstermBackend<Stderr>>, path: Option<&String>) -> Result<()> {
    let mut buffer: TextBuffer = match path {
        Some(path) => fs::read_to_string(path)?.into(),
        None => TextBuffer::default(),
    };
    // Main application loop
    loop {
        // Render the UI
        terminal.draw(|f| {
            let paragraph: Paragraph = buffer.clone().into();
            f.render_widget(paragraph, f.size());
        })?;
        let cursor = buffer.cursor_pos();
        terminal.set_cursor(cursor.0 + 1, cursor.1 + 1)?;
        terminal.show_cursor()?;
        if let Some(_signal) = key::handle_keypress(&mut buffer) {
            if let Some(path) = path {
                let string: String = buffer.into();
                fs::write(path, string)?;
            }
            return Ok(());
        }
    }
}
