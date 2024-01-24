mod key;
pub mod text;

use std::{error::Error, io::stderr, panic};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Block, BorderType, Borders},
};

use text::TextBuffer;

enum SignalToApp {
    Exit,
}

const BLOCK: Block = Block::new()
    .border_type(BorderType::Rounded)
    .borders(Borders::ALL);

fn main() -> Result<(), Box<dyn Error>> {
    init_panic_handler();
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut buffer = TextBuffer::default();
    // Main application loop
    loop {
        // Render the UI
        terminal.draw(|f| {
            f.render_widget(buffer.render().block(BLOCK), f.size());
        })?;
        let cursor = buffer.cursor_pos();
        terminal.set_cursor(cursor.0 + 1, cursor.1 + 1)?;
        terminal.show_cursor()?;
        if let Some(_signal) = key::handle_keypress(&mut buffer) {
            break;
        }
    }

    clean_up()
}

pub fn init_panic_handler() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        clean_up().unwrap();
        original_hook(panic_info);
    }));
}

fn clean_up() -> Result<(), Box<dyn Error>> {
    // shutdown down: reset terminal back to original state
    execute!(stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
